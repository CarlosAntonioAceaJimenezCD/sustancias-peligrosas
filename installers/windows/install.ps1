# Instalador de Sustancias Peligrosas para Windows.
# Descarga/usa el codigo fuente, instala las herramientas necesarias,
# compila la aplicacion Dioxus y la instala en el perfil del usuario.

$ErrorActionPreference = 'Stop'

$RepoUrl    = 'https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas.git'
$AppName    = 'Sustancias Peligrosas'
$ExeName    = 'sustancias-web.exe'
$InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\SustanciasPeligrosas'
$WorkDir    = Join-Path $env:TEMP 'sustancias-peligrosas-build'
$StartMenu  = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs'

function Write-Step {
    param([string]$Mensaje)
    Write-Host "`n==> $Mensaje" -ForegroundColor Cyan
}

function Refresh-Path {
    $machine = [Environment]::GetEnvironmentVariable('Path', 'Machine')
    $user    = [Environment]::GetEnvironmentVariable('Path', 'User')
    $env:Path = "$machine;$user"
}

function Install-WithWinget {
    param([string]$Id)
    if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
        Write-Host "winget no esta disponible; instala $Id manualmente." -ForegroundColor Yellow
        return
    }
    winget install --id $Id -e --accept-source-agreements --accept-package-agreements
    Refresh-Path
}

Write-Host "===================================================" -ForegroundColor Green
Write-Host " Instalador de $AppName (compilacion local)" -ForegroundColor Green
Write-Host "===================================================" -ForegroundColor Green

# ---------------------------------------------------------------- Git
if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Step 'Instalando Git...'
    Install-WithWinget 'Git.Git'
}
if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    throw 'Git no esta disponible. Instalalo y vuelve a ejecutar el instalador.'
}

# ---------------------------------------------------------------- Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Step 'Instalando Rust (toolchain estable, perfil minimo)...'
    $rustup = Join-Path $env:TEMP 'rustup-init.exe'
    Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile $rustup
    & $rustup -y --profile minimal --default-toolchain stable
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    Refresh-Path
}
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Cargo no esta disponible despues de la instalacion.'
}

# ------------------------------------------------ MSVC Build Tools
if (-not (Get-Command cl -ErrorAction SilentlyContinue)) {
    Write-Step 'Instalando Visual Studio Build Tools (carga C++ / MSVC)...'
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        winget install --id Microsoft.VisualStudio.2022.BuildTools -e `
            --accept-source-agreements --accept-package-agreements `
            --override '--wait --add Microsoft.VisualStudio.Workload.VCTools'
        Refresh-Path
    } else {
        Write-Host 'Instala Visual Studio Build Tools con la carga de C++ manualmente.' -ForegroundColor Yellow
    }
}
if (-not (Get-Command cl -ErrorAction SilentlyContinue)) {
    Write-Host "AVISO: no se detecto 'cl' (MSVC). Si la compilacion falla, instala Visual Studio Build Tools con la carga de C++." -ForegroundColor Yellow
}

# ------------------------------------------------ WebView2
Write-Step 'Comprobando WebView2 Runtime...'
Install-WithWinget 'Microsoft.WebView2Runtime'

# ------------------------------------------------ Codigo fuente
Write-Step 'Obteniendo el codigo fuente...'
if (Test-Path (Join-Path $PSScriptRoot 'source\.git')) {
    $SrcDir = Join-Path $PSScriptRoot 'source'
    Write-Host "Usando copia local incluida en el instalador."
} else {
    if (Test-Path $WorkDir) { Remove-Item -Recurse -Force $WorkDir }
    git clone --depth 1 $RepoUrl $WorkDir
    $SrcDir = $WorkDir
}

# ------------------------------------------------ Compilacion
Write-Step 'Compilando la aplicacion (esto puede tardar varios minutos)...'
Push-Location $SrcDir
try {
    & cargo build -p sustancias-web --features desktop --release
    if ($LASTEXITCODE -ne 0) { throw "La compilacion fallo con codigo $LASTEXITCODE" }
    $BuiltExe = Join-Path $SrcDir "target\release\$ExeName"
    if (-not (Test-Path $BuiltExe)) { throw "No se encontro el ejecutable compilado en $BuiltExe" }
} finally {
    Pop-Location
}

# ------------------------------------------------ Instalacion
Write-Step "Instalando en $InstallDir ..."
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item -Force $BuiltExe (Join-Path $InstallDir $ExeName)

# Acceso directo en el menu de inicio
$shortcutPath = Join-Path $StartMenu "$AppName.lnk"
$WshShell = New-Object -ComObject WScript.Shell
$shortcut = $WshShell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = Join-Path $InstallDir $ExeName
$shortcut.WorkingDirectory = $InstallDir
$shortcut.Description = "Marco legal y fichas tecnicas de sustancias peligrosas"
$shortcut.Save()

# Script de desinstalacion
Copy-Item -Force (Join-Path $PSScriptRoot 'desinstalar.ps1') (Join-Path $InstallDir 'desinstalar.ps1') -ErrorAction SilentlyContinue

Write-Host "`nINSTALACION COMPLETADA" -ForegroundColor Green
Write-Host "Ejecutable: $(Join-Path $InstallDir $ExeName)"
Write-Host "Acceso directo: $shortcutPath"
