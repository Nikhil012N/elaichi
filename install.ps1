$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$targetDir = "$env:APPDATA\npm"

Write-Host "[Elaichi Installer] Installing elaichi globally..." -ForegroundColor Cyan

if (-not (Test-Path $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
}

$cmdContent = "@ECHO off`r`nSETLOCAL`r`nnode `"$scriptDir\bin\elaichi.js`" %*"
Set-Content -Path "$targetDir\elaichi.cmd" -Value $cmdContent -Encoding ASCII

$shContent = "#!/usr/bin/env sh`nnode `"$($scriptDir -replace '\\', '/')/bin/elaichi.js`" `"`$@`""
Set-Content -Path "$targetDir\elaichi" -Value $shContent -Encoding ASCII

$ps1Content = "& node `"$scriptDir\bin\elaichi.js`" `$args"
Set-Content -Path "$targetDir\elaichi.ps1" -Value $ps1Content -Encoding ASCII

Write-Host "[Elaichi Installer] Installed successfully!" -ForegroundColor Green
Write-Host "You can now run:" -ForegroundColor Yellow
Write-Host "  elaichi --version"
Write-Host "  elaichi run <file.el>"
