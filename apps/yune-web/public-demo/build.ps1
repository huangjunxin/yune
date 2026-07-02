[CmdletBinding()]
param(
    [string]$OutputDir = ""
)

$ErrorActionPreference = "Stop"

$PublicRoot = $PSScriptRoot
$BuildScript = Join-Path $PublicRoot "build.mjs"
$Node = if ($IsWindows) { "node.exe" } else { "node" }

$Arguments = @($BuildScript)
if (-not [string]::IsNullOrWhiteSpace($OutputDir)) {
    $Arguments += $OutputDir
}

& $Node @Arguments
if ($LASTEXITCODE -ne 0) {
    throw "yune-web public demo build failed"
}
