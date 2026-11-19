$ErrorActionPreference = 'Stop'

$AppName    = 'Sustancias Peligrosas'
$InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\SustanciasPeligrosas'
$StartMenu  = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs'

Write-Host "Desinstalando $AppName..."

if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir
}

$shortcut = Join-Path $StartMenu "$AppName.lnk"
if (Test-Path $shortcut) {
    Remove-Item -Force $shortcut
}

Write-Host 'Desinstalacion completada.'
