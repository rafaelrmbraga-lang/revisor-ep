@echo off
chcp 65001 >nul
cd /d "%~dp0src-tauri"
echo ==== Compilando o Revisor de Execucao Penal (5-15 min na 1a vez) ====
cargo build --release
if errorlevel 1 (
  echo.
  echo [X] A compilacao falhou. Copie TODA a mensagem acima e me envie.
  pause & exit /b 1
)
echo.
echo [OK] Pronto! Abrindo a pasta do executavel...
start "" "%~dp0src-tauri\target\release"
pause
