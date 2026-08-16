# Como funciona a atualização da base (e como publicar uma nova)

## No aplicativo (automático)
1. Ao abrir, o app usa a base LOCAL (se válida) ou a EMBUTIDA no binário.
   Sem internet, nada muda: offline nunca quebra.
2. Em segundo plano, ele consulta o manifest.json no GitHub Releases.
3. Havendo versão mais nova e compatível (schema_version ≤ suportado):
   baixa, confere o SHA-256, valida o JSON, guarda a anterior como
   base-anterior.json (rollback) e troca de forma atômica.
4. Base corrompida no disco vai para quarentena e o app volta à embutida.
Botão "verificar agora" no selo lateral força a checagem.

## Para publicar uma base nova (o curador humano)
1. Edite base.json (aumente meta.versao, ex.: 2026.09.0).
2. `python ferramentas/gerar-manifest.py base.json SEU-USUARIO/revisor-ep`
3. No GitHub → Releases → "Draft new release" → tag (ex.: base-2026.09.0)
   → anexe base.json E manifest.json → Publish.
4. Pronto: todos os aplicativos pegam a nova base ao abrir.

## Antes da publicação do repositório
Troque OWNER em `src-tauri/src/main.rs` (const MANIFEST_URL) pelo seu usuário
do GitHub e recompile. Enquanto isso não é feito, a checagem falha em silêncio
e o app funciona normalmente com a base embutida.

## Schema
Se um dia a ESTRUTURA da base mudar (não só o conteúdo), incrementa-se
schema_version na base e SCHEMA_SUPORTADO em src/motor.rs — versões antigas
do app recusarão a base nova com aviso claro, em vez de calcular errado.
