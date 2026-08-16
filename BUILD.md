# Revisor de Execução Penal — como gerar o .exe (Windows)

O projeto é Tauri v2: interface em HTML/JS (pasta `app/`) + núcleo Rust (pasta `src-tauri/`).
A base de regras v2026.08.2 está EMBUTIDA no binário (`src-tauri/base.json`).
Nenhum dado de assistido é gravado ou transmitido — o PDF é lido em memória e descartado.

## Pré-requisitos (uma vez só, ~15 min)
1. **Rust** — https://rustup.rs → baixe `rustup-init.exe`, Enter em tudo.
2. **Ferramentas C++ do Visual Studio** — o instalador do Rust oferece; aceite
   ("Desktop development with C++"). Necessário para compilar no Windows.
3. **WebView2** — já vem no Windows 10/11 atualizado (o app usa o do sistema; por isso o exe é leve).
4. **Tauri CLI** — no Prompt de Comando:  `cargo install tauri-cli --version "^2"`

## Compilar
```
cd revisor-ep
cargo tauri build
```
O instalador sai em `src-tauri/target/release/bundle/nsis/Revisor de Execução Penal_0.1.0_x64-setup.exe`
e o executável avulso em `src-tauri/target/release/revisor-ep.exe`.

Para testar sem gerar instalador: `cargo tauri dev` (abre a janela em segundos).

## Onde mexer
- Regras/versão da base → `src-tauri/base.json` (recompilar embute a nova).
- Telas/cores → `app/index.html` (as 4 paletas estão no topo do CSS, em `[data-theme=...]`).
- Parser (regex do layout SEEU) → bloco `RX = {...}` no fim do `index.html`.
- Comandos nativos (leitura de PDF etc.) → `src-tauri/src/main.rs`.

## Estado desta versão (v0.1)
- Revisor: upload real, extração determinística (regex sobre o texto do PDF),
  conferência campo a campo, motor com 4 achados (R-10/32/33/34 da base).
- Calculadora: aba Progressão funcional com CONTAGEM CIVIL EXATA e seleção de
  era pela data do fato (Tema 1354). Demais abas: estrutura pronta, lógica na v0.2.
- Toda saída numérica carrega [CONFERIR CÁLCULO]. A ferramenta aponta; quem decide é o defensor.

## Roadmap curto (Fase 5/6)
GitHub privado com Releases da base (atualização sem recompilar) · guia ilustrado
de publicação · monitor semanal de jurisprudência abrindo issues (humano decide).
