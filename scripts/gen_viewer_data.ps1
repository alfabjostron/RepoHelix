# Generates viewer/data.js from the committed fixture log by invoking the
# built repohelix binary and wrapping its viewer JSON as a browser global.
# Run from the repo root:  pwsh -File scripts/gen_viewer_data.ps1

$ErrorActionPreference = 'Stop'
$root = Join-Path $PSScriptRoot '..'
Push-Location $root
try {
    cargo build --quiet
    $bin = Join-Path $root 'target/debug/repohelix.exe'
    if (-not (Test-Path $bin)) { $bin = Join-Path $root 'target/debug/repohelix' }
    $json = & $bin viewer-data --log 'fixtures/nebula.gitlog'
    $out = Join-Path $root 'viewer/data.js'
