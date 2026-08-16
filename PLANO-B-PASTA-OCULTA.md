# PLANO B — subir a pasta oculta SEM vê-la no Explorer

A pasta .github tem só 2 arquivos. Em vez de arrastá-la, vamos CRIÁ-LOS
direto pelo site do GitHub, digitando o caminho (o site cria as pastas
sozinho quando você digita nome com barra). 5 minutos.

── ARQUIVO 1: o robô de compilação ─────────────────────────
1. Na página do seu repositório, clique em "Add file" → "Create new file"
   (botão perto do canto superior direito da lista de arquivos).
2. No campo do nome, digite EXATAMENTE (com as barras):
      .github/workflows/build.yml
   (ao digitar cada "/", o site abre uma pastinha — é assim mesmo)
3. Cole no corpo o CONTEÚDO 1 abaixo → botão verde "Commit changes".

── ARQUIVO 2: o modelo de issue ────────────────────────────
4. De novo "Add file" → "Create new file" → nome:
      .github/ISSUE_TEMPLATE/relato-de-teste.md
5. Cole o CONTEÚDO 2 abaixo → "Commit changes".

Pronto: a pasta oculta existe no GitHub sem nunca ter aparecido no seu
Explorer. Siga para a etapa do Release (o robô já estará ativo — se a aba
Actions pedir para habilitar workflows, clique no botão verde).

════════════════════════════════════════════════════════════
CONTEÚDO 1 — copie TUDO entre as linhas de ~~~ (sem incluí-las)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
name: Compilar Revisor EP (Windows)

# Dois gatilhos:
# 1) Você publica um Release pelo site -> compila e ANEXA exe + base + manifest ao próprio Release.
# 2) Botão manual (aba Actions -> Run workflow) -> compila e deixa o exe para download em "Artifacts".
on:
  release:
    types: [published]
  workflow_dispatch:

permissions:
  contents: write   # necessário para anexar arquivos ao Release

jobs:
  compilar:
    runs-on: windows-latest
    steps:
      - name: Baixar o código
        uses: actions/checkout@v4

      - name: Instalar Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache das dependências (acelera as próximas)
        uses: Swatinem/rust-cache@v2
        with:
          workspaces: src-tauri

      - name: Compilar o executável
        run: cargo build --release --manifest-path src-tauri/Cargo.toml

      - name: Gerar manifest.json da base
        run: python ferramentas/gerar-manifest.py src-tauri/base.json ${{ github.repository }}

      - name: Anexar exe + base + manifest ao Release
        if: github.event_name == 'release'
        uses: softprops/action-gh-release@v2
        with:
          files: |
            src-tauri/target/release/revisor-ep.exe
            src-tauri/base.json
            src-tauri/manifest.json

      - name: Disponibilizar exe como artefato (execução manual)
        if: github.event_name == 'workflow_dispatch'
        uses: actions/upload-artifact@v4
        with:
          name: revisor-ep-windows
          path: |
            src-tauri/target/release/revisor-ep.exe
            src-tauri/base.json
            src-tauri/manifest.json
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

CONTEÚDO 2 — copie TUDO entre as linhas de ~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
---
name: Relato de teste
about: Divergência de cálculo, erro ou sugestão
labels: teste-colegas
---

**O que você estava fazendo?** (ex.: análise no Dec. 12.790, calculadora de prescrição)

**Dados de entrada** (SEM nome real do assistido — use iniciais):
- Ações penais (capitulação, violência, pena):
- Pena cumprida / remição / regime / reincidência no marco:
- Falta grave (data do fato):

**O que o programa mostrou?** (print ajuda)

**O que você esperava?** (se divergência de cálculo: qual o valor correto e por quê —
dispositivo/tema, se souber)

**Versão:** app v0.3 · base v(veja o selo no canto inferior esquerdo)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
