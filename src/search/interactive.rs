use crate::cache::ContentCache;
use crate::cache::types::GistInfo;
use crate::error::{GistCacheError, Result};
use crate::execution::highlight::highlight_content;
use crate::github::GitHubApi;
use console::{Key, Term, style};
use std::collections::HashMap;
use std::path::Path;

const DEFAULT_DESCRIPTION: &str = "No description";
// Reserve space for the "❯ "/"  " cursor prefix and a trailing column so a
// truncated line never wraps, even on narrow terminals.
const RESERVED_WIDTH: usize = 3;
// Truncate short-mode items to this width even on very wide terminals: a
// 130-column single line is hard to scan even when it technically fits, so
// truncation shouldn't only kick in once text overflows the screen.
const MAX_ITEM_WIDTH: usize = 100;

// Enter/leave the terminal's alternate screen buffer (the same mechanism
// vim/less/htop use). Inside it, the "screen" is exactly the visible
// viewport with no scrollback, so a full clear-and-redraw is always safe —
// no more reasoning about how many rows to rewind, which is what caused the
// display corruption for long lists. Leaving it instantly restores whatever
// was on the terminal before, with no cleanup bookkeeping needed.
const ENTER_ALT_SCREEN: &str = "\x1b[?1049h";
const LEAVE_ALT_SCREEN: &str = "\x1b[?1049l";
const CLEAR_AND_HOME: &str = "\x1b[2J\x1b[H";

/// A case-insensitive text matcher for the `/` filter (list) and `/` search
/// (preview). Compiles the pattern as a regex when possible so power users
/// get real regex support; falls back to a literal substring match when the
/// pattern doesn't parse as one — most notably while the user is still
/// mid-typing something like an unbalanced `(`, so the view never goes
/// blank (or panics) just because a pattern isn't finished yet.
enum Matcher {
    Regex(Box<fancy_regex::Regex>),
    Literal(String),
}

impl Matcher {
    fn new(pattern: &str) -> Self {
        match fancy_regex::RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
        {
            Ok(re) => Matcher::Regex(Box::new(re)),
            Err(_) => Matcher::Literal(pattern.to_lowercase()),
        }
    }

    fn is_match(&self, text: &str) -> bool {
        match self {
            Matcher::Regex(re) => re.is_match(text).unwrap_or(false),
            Matcher::Literal(needle) => text.to_lowercase().contains(needle.as_str()),
        }
    }
}

/// Builds the label shown for a gist. Gist descriptions conventionally
/// already start with the primary filename (e.g. "hello.py - a script"), so
/// filenames already present in the description are not repeated.
fn build_item_text(gist: &GistInfo) -> String {
    let desc = gist.description.as_deref().unwrap_or(DEFAULT_DESCRIPTION);
    let desc_lower = desc.to_lowercase();

    let extra_files: Vec<&str> = gist
        .files
        .iter()
        .map(|f| f.filename.as_str())
        .filter(|f| !desc_lower.contains(&f.to_lowercase()))
        .collect();

    if extra_files.is_empty() {
        desc.to_string()
    } else {
        format!("{} - {}", desc, extra_files.join(", "))
    }
}

/// Number of terminal rows `text` occupies when wrapped at column `width`.
fn visual_row_count(text: &str, width: usize) -> usize {
    if width == 0 {
        return 1;
    }
    console::measure_text_width(text).div_ceil(width).max(1)
}

/// Renders one list row, truncating to fit `width` unless `full` is set.
/// Truncation happens on the plain body text before styling is applied, so
/// ANSI codes are never cut mid-sequence.
fn render_line(text: &str, is_selected: bool, width: usize, full: bool) -> String {
    let available = width.min(MAX_ITEM_WIDTH).saturating_sub(RESERVED_WIDTH);
    let body = if full || available == 0 {
        text.to_string()
    } else {
        console::truncate_str(text, available, "...").to_string()
    };

    if is_selected {
        format!("{} {}", style("❯").cyan().bold(), style(body).cyan())
    } else {
        format!("  {}", body)
    }
}

/// Rows available for item rows, after reserving one for the fixed header,
/// one for the filter-status line (always shown, whether or not a filter is
/// active, so the reserved budget never shifts as one gets typed), one for
/// the position footer, and one safety margin row.
fn page_capacity(term_rows: usize) -> usize {
    term_rows.saturating_sub(4).max(1)
}

/// Picks a contiguous range of item indices to display, keeping `selected`
/// visible without the total *rendered* row count (accounting for
/// full-mode lines that wrap to more than one terminal row) exceeding
/// `row_budget`. Grows outward from `selected` — mostly downward, filling
/// upward with whatever budget remains — so item-count-based assumptions
/// from short mode don't silently break once wrapped lines are wider than
/// one row.
fn visible_window(
    items: &[String],
    selected: usize,
    full: bool,
    width: usize,
    row_budget: usize,
) -> std::ops::Range<usize> {
    if items.is_empty() {
        return 0..0;
    }
    let row_budget = row_budget.max(1);

    let row_cost = |i: usize| visual_row_count(&render_line(&items[i], false, width, full), width);

    let mut start = selected;
    let mut end = selected + 1;
    let mut used = row_cost(selected).min(row_budget);

    loop {
        let mut grew = false;

        if end < items.len() {
            let cost = row_cost(end);
            if used + cost <= row_budget {
                used += cost;
                end += 1;
                grew = true;
            }
        }

        if start > 0 {
            let cost = row_cost(start - 1);
            if used + cost <= row_budget {
                used += cost;
                start -= 1;
                grew = true;
            }
        }

        if !grew {
            break;
        }
    }

    start..end
}

/// Fetches one file's content, checking an in-session cache first, then the
/// on-disk content cache, then falling back to the GitHub API — same
/// fallback chain as `ScriptRunner::preview_content`. Successful API fetches
/// are written back to the disk cache (best-effort) and the session cache,
/// so re-opening the same preview during this picker session never refetches.
fn fetch_file_content(
    contents_dir: &Path,
    gist_id: &str,
    filename: &str,
    session_cache: &mut HashMap<(String, String), String>,
) -> Result<String> {
    let key = (gist_id.to_string(), filename.to_string());
    if let Some(cached) = session_cache.get(&key) {
        return Ok(cached.clone());
    }

    let content_cache = ContentCache::new(contents_dir.to_path_buf());
    let content = if content_cache.exists(gist_id, filename) {
        match content_cache.read(gist_id, filename) {
            Ok(c) => c,
            Err(_) => GitHubApi::new().fetch_gist_content(gist_id, filename)?,
        }
    } else {
        let fetched = GitHubApi::new().fetch_gist_content(gist_id, filename)?;
        let _ = content_cache.write(gist_id, filename, &fetched);
        fetched
    };

    session_cache.insert(key, content.clone());
    Ok(content)
}

/// A gist's preview, flattened to individual display lines plus the
/// absolute line index of each file's "--- filename ---" divider, so the
/// viewer can pin the divider of whichever file is currently on screen.
struct PreviewContent {
    lines: Vec<String>,
    dividers: Vec<usize>,
}

/// Builds the full set of display lines for a gist's preview — description,
/// then each file's "--- filename ---" divider and syntax-highlighted
/// content — as individual terminal lines so they can be scrolled. Each
/// highlighted content line gets its own trailing reset code appended: the
/// viewer only ever prints a sub-slice of this list, and without a
/// per-line reset a color set by a line that's scrolled out of view could
/// otherwise bleed into whatever is drawn after it.
fn build_preview_lines(
    gist: &GistInfo,
    contents_dir: &Path,
    session_cache: &mut HashMap<(String, String), String>,
) -> PreviewContent {
    let desc = gist.description.as_deref().unwrap_or(DEFAULT_DESCRIPTION);
    let mut lines = vec![style(desc).cyan().bold().to_string(), String::new()];
    let mut dividers = Vec::new();

    for file in &gist.files {
        dividers.push(lines.len());
        lines.push(
            style(format!("--- {} ---", file.filename))
                .yellow()
                .bold()
                .to_string(),
        );

        match fetch_file_content(contents_dir, &gist.id, &file.filename, session_cache) {
            Ok(content) => {
                let highlighted = highlight_content(&file.filename, &content);
                lines.extend(highlighted.split('\n').map(|line| format!("{line}\x1b[0m")));
            }
            Err(e) => {
                lines.push(
                    style(format!("  Failed to fetch content: {e}"))
                        .red()
                        .to_string(),
                );
            }
        }
        lines.push(String::new());
    }

    PreviewContent { lines, dividers }
}

/// The divider belonging to whichever file section `cursor` currently sits
/// in: the last divider at or before `cursor`, so it's pinned only once the
/// user has scrolled into (or past) that section, and "unsticks" again when
/// scrolling back above it.
fn active_divider(dividers: &[usize], cursor: usize) -> Option<usize> {
    dividers.iter().rev().copied().find(|&d| d <= cursor)
}

/// Clamps a scroll position to a `[range_start, range_end)` sub-range of the
/// content — used instead of always starting at 0 so the scrollable body can
/// begin right after whatever's currently pinned (header + active divider).
fn clamp_scroll(scroll: usize, viewport: usize, range_start: usize, range_end: usize) -> usize {
    let max_scroll = range_end.saturating_sub(viewport).max(range_start);
    scroll.clamp(range_start, max_scroll)
}

/// Moves the viewport's scroll offset just enough to keep `cursor` visible
/// within `[range_start, range_end)`, following it past either edge (same
/// idea as the list's `visible_window`, simplified since every preview line
/// is exactly one row).
fn follow_cursor(
    scroll: usize,
    cursor: usize,
    viewport: usize,
    range_start: usize,
    range_end: usize,
) -> usize {
    let cursor = cursor.clamp(range_start, range_end.saturating_sub(1).max(range_start));
    let scroll = if cursor < scroll {
        cursor
    } else if cursor >= scroll + viewport {
        cursor + 1 - viewport
    } else {
        scroll
    };
    clamp_scroll(scroll, viewport, range_start, range_end)
}

/// Prefixes a preview line with a right-aligned line number and a leading
/// `|` marker on the cursor's row, so the current position is visible
/// alongside the line count. Truncates `content` to fit `width` (accounting
/// for the gutter it just added): without this, a line wider than the
/// terminal wraps, which both throws off the row-budget math that keeps the
/// pinned header on screen and makes the wrapped remainder spill out from
/// under the line-number column instead of staying aligned with it.
fn render_preview_line(
    line_no: usize,
    content: &str,
    is_cursor: bool,
    gutter_width: usize,
    width: usize,
) -> String {
    let marker = if is_cursor { "|" } else { " " };
    let prefix_width = gutter_width + 3; // marker + space + gutter + space
    let available = width.saturating_sub(prefix_width);
    let body = if available == 0 {
        content.to_string()
    } else {
        format!(
            "{}\x1b[0m",
            console::truncate_str(content, available, "...")
        )
    };
    format!("{marker} {line_no:>gutter_width$} {body}")
}

/// Finds the next (or, going backward, previous) line whose plain text
/// (ANSI codes stripped, since `lines` holds highlighted content) matches
/// `matcher`, starting just past `from` and wrapping around the whole
/// document. Returns `None` if nothing matches anywhere, including `from`
/// itself once the search has wrapped all the way around.
fn find_next_match(
    lines: &[String],
    matcher: &Matcher,
    from: usize,
    forward: bool,
) -> Option<usize> {
    let len = lines.len();
    if len == 0 {
        return None;
    }
    let mut i = from;
    for _ in 0..len {
        i = if forward {
            (i + 1) % len
        } else {
            (i + len - 1) % len
        };
        if matcher.is_match(&console::strip_ansi_codes(&lines[i])) {
            return Some(i);
        }
    }
    None
}

/// Shows the full, syntax-highlighted content of `gist` as a scrollable,
/// full-screen viewer with line numbers and a cursor marking the current
/// line. The description and the current file's "--- filename ---" divider
/// stay pinned at the top while the body beneath scrolls, like a frozen
/// table header. A `/` search (regex supported, falls back to a literal
/// substring match) jumps the cursor to the next match; `n`/`N` repeat it
/// forward/backward. Runs entirely inside the alternate screen, so it's safe
/// to display content of any length — each redraw is a full clear, so
/// there's no row-count bookkeeping to get wrong regardless of how far the
/// content scrolls. Returns once the user asks to go back (Space/Esc/Enter).
fn show_preview(
    term: &Term,
    gist: &GistInfo,
    contents_dir: &Path,
    session_cache: &mut HashMap<(String, String), String>,
) -> Result<()> {
    let preview = build_preview_lines(gist, contents_dir, session_cache);
    let lines = &preview.lines;
    let gutter_width = lines.len().max(1).to_string().len();
    // Description + blank line are always pinned; the active file divider
    // joins them once the cursor scrolls into (or past) its section.
    const HEADER_LEN: usize = 2;
    let mut cursor = 0usize;
    let mut scroll = HEADER_LEN;
    let mut search_input = String::new();
    let mut editing_search = false;
    let mut last_search: Option<String> = None;

    loop {
        let (rows, width) = {
            let (r, w) = term.size();
            (r as usize, w as usize)
        };

        let divider = active_divider(&preview.dividers, cursor);
        let pinned_count = HEADER_LEN + if divider.is_some() { 1 } else { 0 };
        let body_start = divider.map(|d| d + 1).unwrap_or(HEADER_LEN);

        // Reserve the pinned rows, one for the footer, one safety margin row.
        let viewport = rows.saturating_sub(pinned_count + 2).max(1);
        scroll = follow_cursor(scroll, cursor, viewport, body_start, lines.len());
        let end = (scroll + viewport).min(lines.len());

        term.write_str(CLEAR_AND_HOME).map_err(GistCacheError::Io)?;

        for (i, line) in lines.iter().enumerate().take(HEADER_LEN) {
            term.write_line(&render_preview_line(
                i + 1,
                line,
                i == cursor,
                gutter_width,
                width,
            ))
            .map_err(GistCacheError::Io)?;
        }
        if let Some(d) = divider {
            term.write_line(&render_preview_line(
                d + 1,
                &lines[d],
                d == cursor,
                gutter_width,
                width,
            ))
            .map_err(GistCacheError::Io)?;
        }

        for (i, line) in lines.iter().enumerate().take(end).skip(scroll) {
            term.write_line(&render_preview_line(
                i + 1,
                line,
                i == cursor,
                gutter_width,
                width,
            ))
            .map_err(GistCacheError::Io)?;
        }

        let footer = if editing_search {
            format!("/{search_input}_")
        } else {
            let search_hint = match &last_search {
                Some(pattern) => format!(", /{pattern} n/N next/prev"),
                None => String::new(),
            };
            format!(
                "-- Line {}/{} -- ↑/↓ move, PgUp/PgDn Home/End jump, / search{search_hint}, Space/Esc/Enter back --",
                cursor + 1,
                lines.len()
            )
        };
        // Truncate rather than let it wrap: a wrapped footer would consume
        // an extra row we didn't budget for, which — on a short terminal —
        // is exactly what pushes the pinned header off the top of the screen.
        term.write_line(
            &style(console::truncate_str(&footer, width, "").into_owned())
                .dim()
                .to_string(),
        )
        .map_err(GistCacheError::Io)?;

        let key = term.read_key().map_err(GistCacheError::Io)?;
        let last_line = lines.len().saturating_sub(1);

        if editing_search {
            match key {
                Key::Enter => {
                    let matcher = Matcher::new(&search_input);
                    if let Some(pos) = find_next_match(lines, &matcher, cursor, true) {
                        cursor = pos;
                    }
                    last_search = Some(std::mem::take(&mut search_input));
                    editing_search = false;
                }
                Key::Escape => {
                    editing_search = false;
                    search_input.clear();
                }
                Key::Backspace => {
                    search_input.pop();
                }
                Key::CtrlC => break,
                Key::Char(c) => search_input.push(c),
                _ => {}
            }
            continue;
        }

        match key {
            Key::ArrowUp => cursor = cursor.saturating_sub(1),
            Key::ArrowDown => cursor = (cursor + 1).min(last_line),
            Key::PageUp => cursor = cursor.saturating_sub(viewport),
            Key::PageDown => cursor = (cursor + viewport).min(last_line),
            Key::Home => cursor = 0,
            Key::End => cursor = last_line,
            Key::Char('/') => editing_search = true,
            Key::Char('n') => {
                if let Some(pattern) = &last_search {
                    let matcher = Matcher::new(pattern);
                    if let Some(pos) = find_next_match(lines, &matcher, cursor, true) {
                        cursor = pos;
                    }
                }
            }
            Key::Char('N') => {
                if let Some(pattern) = &last_search {
                    let matcher = Matcher::new(pattern);
                    if let Some(pos) = find_next_match(lines, &matcher, cursor, false) {
                        cursor = pos;
                    }
                }
            }
            Key::Char(' ') | Key::Char('\u{3000}') | Key::Escape | Key::Enter | Key::CtrlC => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

/// Interactive picker for search results, styled like dialoguer's Select but
/// with a Tab toggle between truncated and full item text (long
/// descriptions/filenames are abbreviated by default so the list stays
/// scannable), a Space toggle to preview the selected gist's full,
/// highlighted content without leaving the picker, and a `/` filter (regex
/// supported, falls back to a literal substring match) to narrow the
/// results further. Scrolls to keep the selection visible when there are
/// more results than fit on screen. Runs inside the terminal's alternate
/// screen buffer, so the picker (and any preview) never leaves artifacts in
/// the shell's scrollback. Returns `Ok(None)` when the user cancels
/// (Esc/Ctrl+C, or Esc a second time once an active filter has already been
/// cleared).
pub fn select(results: &[&GistInfo], contents_dir: &Path) -> Result<Option<usize>> {
    let term = Term::stdout();
    let items: Vec<String> = results.iter().map(|g| build_item_text(g)).collect();
    let debug = std::env::var("GIST_CACHE_VERBOSE").is_ok();
    let mut key_log: Vec<String> = Vec::new();
    let mut content_cache: HashMap<(String, String), String> = HashMap::new();

    term.write_str(ENTER_ALT_SCREEN)
        .map_err(GistCacheError::Io)?;
    term.hide_cursor().map_err(GistCacheError::Io)?;

    let mut selected = 0usize;
    let mut full = false;
    let mut filter_input = String::new();
    let mut editing_filter = false;

    let outcome = loop {
        let matcher = Matcher::new(&filter_input);
        let visible: Vec<usize> = (0..items.len())
            .filter(|&i| matcher.is_match(&items[i]))
            .collect();
        selected = if visible.is_empty() {
            0
        } else {
            selected.min(visible.len() - 1)
        };

        let (rows, width) = {
            let (r, w) = term.size();
            (r as usize, w as usize)
        };
        let row_budget = page_capacity(rows);

        term.write_str(CLEAR_AND_HOME).map_err(GistCacheError::Io)?;

        term.write_line(&format!(
            "{} Select a Gist {}",
            style("?").cyan(),
            style("(↑/↓ move, Space preview, Tab full/short, / filter, Enter select, Esc cancel)")
                .dim()
        ))
        .map_err(GistCacheError::Io)?;

        let filter_status = if editing_filter {
            format!("Filter: /{filter_input}_")
        } else if !filter_input.is_empty() {
            format!(
                "Filter: /{filter_input}  ({}/{} shown, / to edit, Esc to clear)",
                visible.len(),
                items.len()
            )
        } else {
            "Filter: (press / to narrow results)".to_string()
        };
        term.write_line(&style(filter_status).dim().to_string())
            .map_err(GistCacheError::Io)?;

        if visible.is_empty() {
            term.write_line(&style("  -- No matches --").red().to_string())
                .map_err(GistCacheError::Io)?;
        } else {
            let filtered_items: Vec<String> = visible.iter().map(|&i| items[i].clone()).collect();
            let window = visible_window(&filtered_items, selected, full, width, row_budget);
            for i in window {
                term.write_line(&render_line(&filtered_items[i], i == selected, width, full))
                    .map_err(GistCacheError::Io)?;
            }
        }

        term.write_line(
            &style(format!(
                "  [{}/{}] (Tab: {})",
                if visible.is_empty() { 0 } else { selected + 1 },
                visible.len(),
                if full { "full" } else { "short" }
            ))
            .dim()
            .to_string(),
        )
        .map_err(GistCacheError::Io)?;

        let key = match term.read_key() {
            Ok(key) => key,
            Err(e) => {
                term.show_cursor().ok();
                term.write_str(LEAVE_ALT_SCREEN).ok();
                return Err(GistCacheError::Io(e));
            }
        };

        if debug {
            key_log.push(format!("{:?}", key));
        }

        if editing_filter {
            match key {
                Key::Enter => {
                    if let Some(&i) = visible.get(selected) {
                        break Some(i);
                    }
                }
                Key::Escape => editing_filter = false,
                Key::Backspace => {
                    filter_input.pop();
                }
                Key::ArrowUp if !visible.is_empty() => {
                    selected = if selected == 0 {
                        visible.len() - 1
                    } else {
                        selected - 1
                    };
                }
                Key::ArrowDown if !visible.is_empty() => {
                    selected = (selected + 1) % visible.len();
                }
                Key::Tab => full = !full,
                Key::CtrlC => break None,
                Key::Char(c) => filter_input.push(c),
                _ => {}
            }
        } else {
            match key {
                Key::Char('/') => editing_filter = true,
                Key::ArrowUp if !visible.is_empty() => {
                    selected = if selected == 0 {
                        visible.len() - 1
                    } else {
                        selected - 1
                    };
                }
                Key::ArrowDown if !visible.is_empty() => {
                    selected = (selected + 1) % visible.len();
                }
                Key::Tab => full = !full,
                // '\u{3000}' (IDEOGRAPHIC SPACE) is what some Japanese IMEs
                // send for the space bar even outside of text conversion, so
                // both are accepted as the preview trigger.
                Key::Char(' ') | Key::Char('\u{3000}') if !visible.is_empty() => {
                    // Preview errors (e.g. a failed API fetch) are shown
                    // inline by show_preview itself; nothing more to do
                    // here either way.
                    let _ = show_preview(
                        &term,
                        results[visible[selected]],
                        contents_dir,
                        &mut content_cache,
                    );
                }
                Key::Enter => {
                    if let Some(&i) = visible.get(selected) {
                        break Some(i);
                    }
                }
                Key::Escape => {
                    if filter_input.is_empty() {
                        break None;
                    }
                    filter_input.clear();
                }
                Key::CtrlC => break None,
                _ => {}
            }
        }
    };

    term.show_cursor().ok();
    term.write_str(LEAVE_ALT_SCREEN)
        .map_err(GistCacheError::Io)?;

    if debug {
        eprintln!("[gist-cache-rs] keys received: {}", key_log.join(", "));
    }

    if let Some(index) = outcome {
        println!(
            "{} Select a Gist {} {}",
            style("✔").green(),
            style("·").dim(),
            items[index]
        );
    }

    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::types::GistFile;
    use chrono::Utc;

    fn gist(desc: Option<&str>, filenames: Vec<&str>) -> GistInfo {
        GistInfo {
            id: "abc123".to_string(),
            description: desc.map(|s| s.to_string()),
            files: filenames
                .into_iter()
                .map(|name| GistFile {
                    filename: name.to_string(),
                    language: None,
                    size: 100,
                })
                .collect(),
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/abc123".to_string(),
        }
    }

    #[test]
    fn matcher_does_plain_case_insensitive_substring_matching() {
        let m = Matcher::new("Hello");
        assert!(m.is_match("say hello world"));
        assert!(!m.is_match("goodbye world"));
    }

    #[test]
    fn matcher_supports_regex_patterns() {
        let m = Matcher::new(r"^test_.*\.py$");
        assert!(m.is_match("test_python.py"));
        assert!(!m.is_match("test_python.rb"));
        assert!(!m.is_match("my_test_python.py"));
    }

    #[test]
    fn matcher_empty_pattern_matches_everything() {
        let m = Matcher::new("");
        assert!(m.is_match("anything"));
        assert!(m.is_match(""));
    }

    #[test]
    fn matcher_falls_back_to_literal_on_invalid_regex() {
        // An unbalanced group is invalid regex syntax (e.g. mid-typing);
        // it must still work as a literal substring match rather than
        // making every item disappear from the list.
        let m = Matcher::new("test_(unclosed");
        assert!(m.is_match("test_(unclosed_group.py"));
        assert!(!m.is_match("unrelated.py"));
    }

    #[test]
    fn find_next_match_wraps_forward() {
        let lines = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        let m = Matcher::new("alpha");
        // Starting past the only match, searching forward wraps around.
        assert_eq!(find_next_match(&lines, &m, 1, true), Some(0));
    }

    #[test]
    fn find_next_match_wraps_backward() {
        let lines = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        let m = Matcher::new("gamma");
        assert_eq!(find_next_match(&lines, &m, 0, false), Some(2));
    }

    #[test]
    fn find_next_match_returns_none_when_nothing_matches() {
        let lines = vec!["alpha".to_string(), "beta".to_string()];
        let m = Matcher::new("zzz");
        assert_eq!(find_next_match(&lines, &m, 0, true), None);
    }

    #[test]
    fn find_next_match_ignores_ansi_codes_in_the_line() {
        let lines = vec![format!("{}needle{}", "\x1b[31m", "\x1b[0m")];
        let m = Matcher::new("needle");
        assert_eq!(find_next_match(&lines, &m, 0, true), Some(0));
    }

    #[test]
    fn build_item_text_omits_filename_already_in_description() {
        let g = gist(
            Some("hello_args.py - Python引数テストスクリプト #python #test"),
            vec!["hello_args.py"],
        );
        assert_eq!(
            build_item_text(&g),
            "hello_args.py - Python引数テストスクリプト #python #test"
        );
    }

    #[test]
    fn build_item_text_appends_filenames_not_in_description() {
        let g = gist(Some("A helper script"), vec!["helper.sh"]);
        assert_eq!(build_item_text(&g), "A helper script - helper.sh");
    }

    #[test]
    fn build_item_text_appends_only_missing_files_from_multiple() {
        let g = gist(Some("main.py - entry point"), vec!["main.py", "utils.py"]);
        assert_eq!(build_item_text(&g), "main.py - entry point - utils.py");
    }

    #[test]
    fn build_item_text_uses_default_when_no_description() {
        let g = gist(None, vec!["script.sh"]);
        assert_eq!(build_item_text(&g), "No description - script.sh");
    }

    #[test]
    fn visual_row_count_wraps_at_width() {
        assert_eq!(visual_row_count("hello", 80), 1);
        assert_eq!(visual_row_count(&"x".repeat(85), 80), 2);
        assert_eq!(visual_row_count("", 80), 1);
    }

    #[test]
    fn visual_row_count_handles_zero_width() {
        assert_eq!(visual_row_count("hello", 0), 1);
    }

    #[test]
    fn render_line_truncates_when_not_full() {
        let text = "a".repeat(100);
        let line = render_line(&text, false, 20, false);
        assert!(console::measure_text_width(&line) <= 20);
        assert!(line.contains("..."));
    }

    #[test]
    fn render_line_truncates_long_text_even_on_a_wide_terminal() {
        // A single long line is hard to scan even when the terminal is wide
        // enough to fit it without wrapping, so short mode caps at
        // MAX_ITEM_WIDTH regardless of the actual terminal width.
        let text = "a".repeat(200);
        let line = render_line(&text, false, 300, false);
        assert!(console::measure_text_width(&line) <= MAX_ITEM_WIDTH);
        assert!(line.contains("..."));
    }

    #[test]
    fn render_line_shows_everything_when_full() {
        let text = "a".repeat(100);
        let line = render_line(&text, false, 20, true);
        assert!(!line.contains("..."));
        assert!(line.ends_with(&text));
    }

    #[test]
    fn render_line_marks_selected_item() {
        let line = render_line("item", true, 80, false);
        assert!(line.contains('❯'));
    }

    #[test]
    fn page_capacity_reserves_header_footer_and_margin() {
        assert_eq!(page_capacity(24), 20);
        assert_eq!(page_capacity(3), 1); // never collapses to zero
        assert_eq!(page_capacity(0), 1);
    }

    #[test]
    fn clamp_scroll_keeps_position_within_bounds() {
        assert_eq!(clamp_scroll(5, 10, 0, 100), 5); // within range, unchanged
        assert_eq!(clamp_scroll(95, 10, 0, 100), 90); // clamps to the last full page
        assert_eq!(clamp_scroll(5, 20, 0, 10), 0); // content fits entirely: no scroll
        assert_eq!(clamp_scroll(0, 5, 0, 0), 0); // empty content
    }

    #[test]
    fn clamp_scroll_never_goes_below_range_start() {
        // Sub-range starting at 4 (e.g. body right after a pinned divider).
        assert_eq!(clamp_scroll(0, 3, 4, 20), 4);
        assert_eq!(clamp_scroll(2, 3, 4, 20), 4);
    }

    #[test]
    fn follow_cursor_scrolls_down_past_bottom_edge() {
        // Viewport [0, 3), cursor moves to line 5: viewport must include it.
        let scroll = follow_cursor(0, 5, 3, 0, 10);
        assert_eq!(scroll, 3);
    }

    #[test]
    fn follow_cursor_scrolls_up_past_top_edge() {
        let scroll = follow_cursor(5, 2, 3, 0, 10);
        assert_eq!(scroll, 2);
    }

    #[test]
    fn follow_cursor_keeps_scroll_when_cursor_stays_in_view() {
        let scroll = follow_cursor(2, 3, 3, 0, 10);
        assert_eq!(scroll, 2);
    }

    #[test]
    fn follow_cursor_never_scrolls_past_the_end() {
        let scroll = follow_cursor(0, 9, 3, 0, 10);
        assert_eq!(scroll, 7); // window becomes [7, 8, 9]
    }

    #[test]
    fn follow_cursor_respects_a_sub_range_start() {
        // Cursor is inside the pinned area (before range_start): the body
        // viewport should still start at range_start, not run before it.
        let scroll = follow_cursor(10, 1, 3, 4, 20);
        assert_eq!(scroll, 4);
    }

    #[test]
    fn active_divider_picks_the_last_divider_at_or_before_cursor() {
        let dividers = [2usize, 10, 18];
        assert_eq!(active_divider(&dividers, 0), None); // still in the header
        assert_eq!(active_divider(&dividers, 2), Some(2)); // on the divider itself
        assert_eq!(active_divider(&dividers, 9), Some(2));
        assert_eq!(active_divider(&dividers, 10), Some(10));
        assert_eq!(active_divider(&dividers, 25), Some(18));
    }

    #[test]
    fn render_preview_line_marks_only_the_cursor_row() {
        let cursor_line = render_preview_line(3, "def main():", true, 3, 80);
        let other_line = render_preview_line(2, "import sys", false, 3, 80);
        assert!(cursor_line.starts_with('|'));
        assert!(other_line.starts_with(' '));
        assert!(cursor_line.contains("  3"));
        assert!(cursor_line.contains("def main():"));
    }

    #[test]
    fn render_preview_line_truncates_to_fit_the_terminal_width() {
        // A narrow terminal must never let the content push the line past
        // `width`: a wrapped line breaks the row-budget math that keeps the
        // pinned header on screen.
        let long_content = "x".repeat(200);
        let line = render_preview_line(1, &long_content, false, 3, 20);
        assert!(console::measure_text_width(&line) <= 20);
        assert!(line.contains("..."));
    }

    #[test]
    fn render_preview_line_keeps_short_content_untruncated() {
        let line = render_preview_line(1, "short", false, 3, 80);
        assert!(line.contains("short"));
        assert!(!line.contains("..."));
    }

    fn make_items(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("item{i}")).collect()
    }

    fn window_row_cost(
        items: &[String],
        window: std::ops::Range<usize>,
        full: bool,
        width: usize,
    ) -> usize {
        window
            .map(|i| visual_row_count(&render_line(&items[i], false, width, full), width))
            .sum()
    }

    #[test]
    fn visible_window_includes_selection_and_respects_row_budget() {
        let items = make_items(10);
        let window = visible_window(&items, 5, false, 80, 3);
        assert!(window.contains(&5));
        assert!(window_row_cost(&items, window, false, 80) <= 3);
    }

    #[test]
    fn visible_window_does_not_run_past_start_of_list() {
        let items = make_items(10);
        let window = visible_window(&items, 0, false, 80, 3);
        assert_eq!(window.start, 0);
    }

    #[test]
    fn visible_window_does_not_run_past_end_of_list() {
        let items = make_items(10);
        let window = visible_window(&items, 9, false, 80, 3);
        assert_eq!(window.end, 10);
    }

    #[test]
    fn visible_window_shows_everything_when_list_fits_budget() {
        let items = make_items(5);
        let window = visible_window(&items, 2, false, 80, 10);
        assert_eq!(window, 0..5);
    }

    #[test]
    fn visible_window_handles_empty_list() {
        let items: Vec<String> = Vec::new();
        assert_eq!(visible_window(&items, 0, false, 80, 10), 0..0);
    }

    #[test]
    fn visible_window_accounts_for_wrapped_rows_in_full_mode() {
        // Each item wraps to multiple rows once rendered in full mode on a
        // narrow terminal, so an item-count-based window would blow the row
        // budget; the window must shrink to compensate.
        let items: Vec<String> = (0..10).map(|_| "a".repeat(50)).collect();
        let width = 20;
        let window = visible_window(&items, 5, true, width, 6);
        assert!(window.contains(&5));
        assert!(window_row_cost(&items, window, true, width) <= 6);
    }
}
