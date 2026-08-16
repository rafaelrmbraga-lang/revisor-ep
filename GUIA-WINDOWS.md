# Gerar o .exe no Windows — guia à prova de tropeços

Você NÃO precisa do "tauri-cli". O executável sai com o Rust puro.
São 2 instalações + 2 cliques.

─────────────────────────────────────────────
PASSO 1 · Instalar o Rust (uma vez só)
─────────────────────────────────────────────
1. Baixe e abra: https://win.rustup.rs/x86_64  (arquivo rustup-init.exe)
2. Na tela preta que abre, se ele oferecer instalar os
   "Visual Studio C++ build tools", ACEITE (digite 1 e Enter).
   → Essa parte baixa ~2 GB e demora. É normal.
3. Ao final, FECHE e ABRA de novo o Prompt de Comando
   (senão o Windows não enxerga o cargo).

Teste: abra o Prompt (tecla Windows → digite cmd → Enter) e rode:
    cargo --version
Deve responder algo como "cargo 1.8x". Se disser
"'cargo' não é reconhecido...", o Prompt é antigo — feche e abra outro.

─────────────────────────────────────────────
PASSO 2 · Compilar (2 cliques)
─────────────────────────────────────────────
1. Extraia o revisor-ep-v0.1.zip (ex.: para a Área de Trabalho).
2. Entre na pasta revisor-ep e dê 2 cliques em:
       1-verificar-ambiente.bat   → confere se está tudo pronto
       2-compilar.bat             → compila (5–15 min na 1ª vez) e
                                    abre a pasta com o revisor-ep.exe
O exe final fica em:  src-tauri\target\release\revisor-ep.exe
É esse arquivo que você copia para onde quiser. Pronto.

─────────────────────────────────────────────
Erros comuns → solução
─────────────────────────────────────────────
• "'cargo' não é reconhecido"      → Rust não instalado OU Prompt aberto
  antes da instalação. Refaça o Passo 1 e abra um Prompt NOVO.
• "linker `link.exe` not found"    → faltaram as ferramentas C++.
  Baixe https://aka.ms/vs/17/release/vs_BuildTools.exe , marque
  "Desktop development with C++", instale, reinicie o Prompt.
• "could not find `Cargo.toml`"    → você rodou o comando na pasta errada.
  Use os .bat (eles entram na pasta certa sozinhos).
• Janela abre em branco            → WebView2 ausente (raro no Win10/11):
  https://developer.microsoft.com/microsoft-edge/webview2/ → "Evergreen".
• Antivírus reclama do exe recém-compilado → falso positivo comum com
  binários novos sem assinatura; adicione exceção à pasta target\release.

(Quando quiser também o INSTALADOR .exe-setup, aí sim:
 cargo install tauri-cli --locked  e depois  cargo tauri build
 — mas isso é opcional e pode ficar para depois.)
