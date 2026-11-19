@echo off
chcp 65001 >nul
title Instalador de Sustancias Peligrosas
echo.
echo  Instalador de Sustancias Peligrosas
echo  Este instalador compila la aplicacion en tu equipo.
echo.
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1"
echo.
pause
