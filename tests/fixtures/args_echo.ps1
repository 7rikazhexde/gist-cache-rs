param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$Arguments
)

if ($Arguments) {
    Write-Host "Arguments: $($Arguments -join ' ')"
} else {
    Write-Host "No arguments provided"
}
