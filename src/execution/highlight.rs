use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

// syntect's bundled defaults are missing several languages this project
// supports (TypeScript, TOML, ...); two-face ships bat's much larger
// syntax collection instead.
static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(two_face::syntax::extra_newlines);

static THEME: LazyLock<Theme> = LazyLock::new(|| {
    let mut themes = ThemeSet::load_defaults();
    themes
        .themes
        .remove("base16-ocean.dark")
        .expect("bundled base16-ocean.dark theme should exist")
});

/// Resolves a syntax by file extension, falling back to shebang-line
/// detection (using in-memory `content`, not disk I/O) and finally to
/// plain text when nothing matches.
fn syntax_for(filename: &str, content: &str) -> &'static SyntaxReference {
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str());

    if let Some(ext) = ext
        && let Some(syntax) = SYNTAX_SET.find_syntax_by_extension(ext)
    {
        return syntax;
    }

    SYNTAX_SET
        .find_syntax_by_first_line(content)
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
}

/// Renders `content` as ANSI 24-bit colored text, using syntax rules
/// inferred from `filename`'s extension. Falls back to the plain-text
/// syntax (no highlighting) when the extension isn't recognized.
pub fn highlight_content(filename: &str, content: &str) -> String {
    let syntax = syntax_for(filename, content);
    let mut highlighter = HighlightLines::new(syntax, &THEME);
    let mut out = String::with_capacity(content.len() + 16);

    for line in LinesWithEndings::from(content) {
        match highlighter.highlight_line(line, &SYNTAX_SET) {
            Ok(ranges) => out.push_str(&as_24_bit_terminal_escaped(&ranges[..], false)),
            Err(_) => out.push_str(line),
        }
    }
    out.push_str("\x1b[0m");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight_content_wraps_output_in_reset_code() {
        let result = highlight_content("script.py", "print(\"hi\")\n");
        assert!(result.ends_with("\x1b[0m"));
        assert!(result.contains("print"));
    }

    #[test]
    fn highlight_content_falls_back_for_unknown_extension() {
        let result = highlight_content("file.unknownext", "plain text content\n");
        assert!(result.contains("plain text content"));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn highlight_content_handles_empty_content() {
        let result = highlight_content("script.sh", "");
        assert_eq!(result, "\x1b[0m");
    }

    #[test]
    fn syntax_for_resolves_known_extensions() {
        assert_eq!(syntax_for("main.rs", "").name, "Rust");
        assert_eq!(syntax_for("script.py", "").name, "Python");
        assert_eq!(syntax_for("script.ts", "").name, "TypeScript");
        assert_eq!(syntax_for("Cargo.toml", "").name, "TOML");
    }

    #[test]
    fn syntax_for_falls_back_to_shebang_when_extension_unknown() {
        let syntax = syntax_for("hello", "#!/usr/bin/env python3\nprint(1)\n");
        assert_eq!(syntax.name, "Python");
    }

    #[test]
    fn syntax_for_does_not_touch_disk_for_nonexistent_files() {
        // Regression test: find_syntax_for_file() opens the path from disk
        // to sniff the shebang, which fails for in-memory-only content.
        // syntax_for() must resolve purely from the filename + content.
        let syntax = syntax_for("does-not-exist-on-disk.py", "print(1)\n");
        assert_eq!(syntax.name, "Python");
    }
}
