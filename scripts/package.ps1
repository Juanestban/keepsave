<#
.SYNOPSIS
  Packages a release build into target\dist: the MSI, the portable .exe and
  SHA256SUMS.txt, named the way the in-app updater expects.

.DESCRIPTION
  Run `cargo build --release -p keepsave-app` first. Needs WiX v5 on PATH:
    dotnet tool install --global wix --version 5.0.2
#>
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot

$metadata = cargo metadata --format-version 1 --no-deps --manifest-path "$root\Cargo.toml" | ConvertFrom-Json
$version = ($metadata.packages | Where-Object { $_.name -eq 'keepsave-app' }).version
# MSI versions are numbers only: 1.1.0-rc.1 installs as 1.1.0.
$msiVersion = ($version -split '[-+]')[0]

$exe = Join-Path $root 'target\release\keepsave.exe'
if (-not (Test-Path $exe)) { throw "missing $exe; run cargo build --release -p keepsave-app" }

$dist = Join-Path $root 'target\dist'
New-Item -ItemType Directory -Force -Path $dist | Out-Null
Get-ChildItem $dist | Remove-Item -Force

$msi = Join-Path $dist "keepsave-$version-x64.msi"
$portable = Join-Path $dist "keepsave-$version-x64-portable.exe"

wix build (Join-Path $root 'crates\keepsave-app\installer\keepsave.wxs') -arch x64 `
  -d "Version=$msiVersion" -d "ExePath=$exe" -d "IconPath=$(Join-Path $root 'crates\keepsave-app\assets\icon\keepsave.ico')" `
  -o $msi
if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
# wix leaves the debug symbols next to the MSI.
Remove-Item (Join-Path $dist '*.wixpdb') -ErrorAction SilentlyContinue

Copy-Item $exe $portable

# "<hash>  <name>", like sha256sum, which the updater parses.
$sums = Get-ChildItem $dist -File | Where-Object { $_.Extension -in '.msi', '.exe' } | Sort-Object Name | ForEach-Object {
  "$((Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLower())  $($_.Name)"
}
[System.IO.File]::WriteAllText((Join-Path $dist 'SHA256SUMS.txt'), (($sums -join "`n") + "`n"))

Get-ChildItem $dist | Format-Table Name, Length
