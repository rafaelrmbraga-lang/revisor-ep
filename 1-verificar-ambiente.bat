@echo off
chcp 65001 >nul
echo ==== Revisor EP · verificacao do ambiente ====
where cargo >nul 2>nul
if errorlevel 1 (
  echo [X] Rust/cargo NAO encontrado. Instale em https://win.rustup.rs/x86_64
  echo     e abra um Prompt NOVO depois da instalacao.
  goto fim
)
for /f "tokens=*" %%v in ('cargo --version') do echo [OK] %%v
where link.exe >nul 2>nul
if errorlevel 1 (
  echo [!] link.exe nao esta no PATH deste prompt. Normalmente o cargo
  echo     encontra sozinho. Se a compilacao falhar com "linker not found",
  echo     instale as ferramentas C++: https://aka.ms/vs/17/release/vs_BuildTools.exe
) else (
  echo [OK] linker do Visual Studio encontrado
)
if exist "%~dp0src-tauri\Cargo.toml" (echo [OK] projeto encontrado em %~dp0src-tauri) else (echo [X] Rode este .bat de dentro da pasta revisor-ep)
echo.
echo Se apareceu [OK] no cargo e no projeto, rode o 2-compilar.bat
:fim
pause
