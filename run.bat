@echo off
title 🚀 Lancement du moteur vivant AURORAE++
echo.
echo [AURORAE++] Compilation en cours...
cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo ❌ Erreur de compilation.
    pause
    exit /b
)

echo.
echo ✅ Compilation réussie.
echo [AURORAE++] Démarrage du système...
echo.

target\release\aurorae.exe

echo.
echo [AURORAE++] Terminé.
pause
