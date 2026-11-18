@echo off
setlocal
set SCRIPT_DIR=%~dp0
set TARGET_DIR=%APPDATA%\npm

echo [Elaichi Installer] Installing elaichi globally...

if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%"
)

(
echo @ECHO off
echo SETLOCAL
echo node "%SCRIPT_DIR%bin\elaichi.js" %%*
) > "%TARGET_DIR%\elaichi.cmd"

(
echo #!/usr/bin/env sh
echo node "%SCRIPT_DIR%bin/elaichi.js" "$@"
) > "%TARGET_DIR%\elaichi"

(
echo ^& node "%SCRIPT_DIR%bin\elaichi.js" $args
) > "%TARGET_DIR%\elaichi.ps1"

echo [Elaichi Installer] Installed successfully!
echo You can now run:
echo   elaichi --version
echo   elaichi run ^<file.el^>
