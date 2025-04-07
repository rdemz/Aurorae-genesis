@echo off

REM ╔═══════════════════════════════════════╗
REM ║       LANCEMENT D'AURORAE++           ║
REM ╚═══════════════════════════════════════╝

cd /d %~dp0

echo.
echo [AURORAE++] 🚀 Compilation et démarrage...
echo.

cargo run --release --manifest-path Cargo.toml

pause
