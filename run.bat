@echo off
setlocal

:: Chargement de l'environnement
echo [AURORAE++] Chargement du .env...
if exist .env (
    for /f "usebackq delims=" %%a in (".env") do set "%%a"
) else (
    echo [AURORAE++] ⚠️ .env manquant, certaines variables pourraient échouer.
)

:: Compilation
echo [AURORAE++] 🔧 Compilation en cours...
cargo build --release
if %errorlevel% neq 0 (
    echo [AURORAE++] ❌ Échec de la compilation
    exit /b 1
)

:: Exécution
echo [AURORAE++] 🚀 Lancement du moteur vivant...
target\release\aurorae.exe

endlocal
pause
