# Subir ao GitHub e distribuir aos colegas — passo a passo

Tudo pelo NAVEGADOR, sem linha de comando de git. Tempo: ~20 minutos.

⚠ DECISÃO IMPORTANTE ANTES DE COMEÇAR: o repositório precisa ser **PÚBLICO**
para o atualizador automático funcionar (o app baixa a base dos Releases sem
senha; em repositório privado isso é bloqueado pelo GitHub). O conteúdo é só
código e legislação — nenhum dado de assistido vai para lá (o .gitignore já
bloqueia PDFs por precaução). Se preferir privado mesmo assim: o app funciona
normalmente, apenas sem atualização automática da base.

─────────────────────────────────────
ETAPA 1 · Conta e repositório (5 min)
─────────────────────────────────────
1. https://github.com/signup → crie a conta (anote o NOME DE USUÁRIO).
2. Logado, clique no "+" no canto superior direito → "New repository".
3. Repository name: revisor-ep · marque "Public" · NÃO marque "Add a README"
   → botão verde "Create repository".

─────────────────────────────────────
ETAPA 2 · Um único ajuste no projeto (1 min, no Bloco de Notas)
─────────────────────────────────────
1. Abra src-tauri\src\main.rs no Bloco de Notas.
2. Localize a linha com OWNER:
   https://github.com/OWNER/revisor-ep/releases/latest/download/manifest.json
   e troque OWNER pelo seu nome de usuário do GitHub. Salve.
(NÃO precisa compilar nada: a nuvem compila — Etapa 4.)

─────────────────────────────────────
ETAPA 3 · Subir o código (5 min)
─────────────────────────────────────
1. Na página do repositório vazio, clique em "uploading an existing file".
2. Arraste TODO o conteúdo da pasta revisor-ep (app, src-tauri, ferramentas,
   os .bat e os .md) — MENOS a pasta src-tauri\target (é pesada e não deve
   subir; se copiou, exclua antes).
   Dica: selecione tudo dentro de revisor-ep e arraste de uma vez; o GitHub
   preserva as subpastas.
3. Em "Commit changes", escreva "versão inicial v0.3" → "Commit changes".

─────────────────────────────────────
ETAPA 4 · Primeiro Release — A NUVEM COMPILA PARA VOCÊ (5 min + espera)
─────────────────────────────────────
Você NÃO precisa compilar em lugar nenhum. O projeto inclui um robô
(.github/workflows/build.yml) que compila num Windows do próprio GitHub.

1. Aba "Actions" do repositório → se aparecer um aviso verde pedindo para
   habilitar workflows, clique em "I understand... enable them".
2. Aba "Releases" → "Create a new release".
3. "Choose a tag" → digite v0.3 → "Create new tag on publish".
4. Title: Revisor EP v0.3 (primeiro ciclo de testes) → "Publish release".
5. Aguarde: aba "Actions" mostra "Compilar Revisor EP (Windows)" rodando
   (10–15 min na primeira vez; depois ~3 min, por causa do cache).
   Ao terminar, volte ao Release: o robô ANEXOU sozinho
   revisor-ep.exe + base.json + manifest.json.
6. PRONTO. Link de download (para você no PC da Defensoria e para os colegas):
   https://github.com/SEU-USUARIO/revisor-ep/releases/latest
   → baixe o revisor-ep.exe e execute. Sem instalação, sem administrador.

Dica: quiser só testar a compilação sem criar Release → aba Actions →
"Compilar Revisor EP (Windows)" → botão "Run workflow" → ao final, o exe
fica em "Artifacts" da execução.

─────────────────────────────────────
ETAPA 5 · Atualizações daqui em diante
─────────────────────────────────────
- SÓ BASE (nova jurisprudência/decreto): edite base.json direto no site do
  GitHub (abrir o arquivo → lápis → suba meta.versao → Commit) e publique um
  novo Release (Etapa 4, itens 2-4). O robô compila e anexa tudo; os apps dos
  colegas atualizam a base sozinhos ao abrir — sem baixar exe novo.
- APP NOVO (mudança de telas/motor): mesmo caminho — novo Release, robô anexa
  o exe novo; avise os colegas para baixar (o binário não se autoatualiza,
  de propósito nesta fase).

─────────────────────────────────────
Problemas comuns
─────────────────────────────────────
- O robô da aba Actions falhou (X vermelho) → clique na execução, copie as
  últimas linhas vermelhas do log e me envie: eu corrijo o workflow.
- Upload recusa a pasta target → correto, ela não deve subir (só o exe vai,
  e vai no Release, não no código).
- Colega diz que o Windows bloqueou o exe → SmartScreen: "Mais informações"
  → "Executar assim mesmo" (binário sem assinatura digital; esperado).

─────────────────────────────────────
SmartScreen: como conviver e como eliminar
─────────────────────────────────────
POR QUE aparece: o exe é novo e não tem assinatura digital; o Windows
desconfia de qualquer binário sem "reputação". Não é vírus, é burocracia.

NO DIA A DIA (por arquivo baixado — 5 segundos):
  Clique com o BOTÃO DIREITO no revisor-ep.exe baixado → Propriedades →
  na parte de baixo da aba Geral, marque "Desbloquear" → OK.
  Isso remove a marca de "arquivo vindo da internet" e o SmartScreen não
  pergunta mais. Alternativa na hora do aviso: "Mais informações" →
  "Executar assim mesmo" (só na primeira execução daquele arquivo).

IMPORTANTE: cada Release gera um exe NOVO (reputação zera). Repita o
"Desbloquear" a cada versão baixada — ou elimine de vez com assinatura:

PARA ELIMINAR DE VEZ (em ordem de custo):
  1. TI da Defensoria (recomendado p/ o PC funcional): pedir liberação do
     executável por hash/pasta (AppLocker/WDAC). E-mail de 3 linhas resolve.
  2. SignPath Foundation (gratuito p/ projetos de código aberto): assina os
     binários do repositório público; SmartScreen passa a confiar.
     https://signpath.org → "Open Source"
  3. Certificado de assinatura de código pago (OV ~US$100-400/ano) ou
     Azure Trusted Signing (~US$10/mês): reputação rápida e sem avisos.
NUNCA: desativar o SmartScreen do sistema — desnecessário e, em máquina
institucional, provavelmente bloqueado por política.
