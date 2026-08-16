# Revisor de Execução Penal

Ferramenta local e offline de apoio à execução penal: análise estruturada de
**indulto e comutação** (Decretos 12.790/2025 e 12.338/2024, inclusive lado a
lado, cada um com a fotografia do seu marco), calculadoras de **progressão,
prescrição executória, livramento condicional, remição/detração e unificação**,
e leitura do RSPE/atestado do SEEU para pré-preenchimento.

## Princípios
- **Nada sai da sua máquina.** O PDF é lido em memória e descartado; não há
  conta, nuvem, telemetria ou banco de dados de assistidos.
- **Nenhum número sem fundamento.** Cada regra da base cita o dispositivo,
  o tema ou a súmula; a base foi construída com verificação ativa de
  legislação e precedentes (Temas 1354, 1195 retificado, 1084, 1165, 709…).
- **A decisão é do defensor.** Toda saída carrega [CONFERIR CÁLCULO] —
  a ferramenta aponta indícios e teses; quem assina, confere.
- **Offline nunca quebra.** A base de regras vem embutida; atualizações são
  baixadas com verificação SHA-256 e rollback automático (ver ATUALIZACAO.md).

## Instalar (Windows)
Baixe o `revisor-ep.exe` na aba **Releases** e execute. Sem instalação, sem
administrador. (Se o SmartScreen alertar — binário novo sem assinatura —
clique em "Mais informações → Executar assim mesmo".)

## Compilar do código
Veja `GUIA-WINDOWS.md` (2 pré-requisitos + 2 cliques nos `.bat`).

## Estado (v0.3) e limites conhecidos
Motor de indulto/comutação: art. 1º por capitulação declarada; art. 6º com o
Tema 1195 retificado e o parágrafo único; pedágio do art. 7º; art. 9º, incisos
I–III, VII e VIII; art. 13, §§ 1º, 2º e 5º. **Ainda não cobertos:** incisos
IV–VI e IX–XVI do art. 9º, redutor do § 2º, arts. 10–12 (mulheres e multa).
Progressão: 3 eras + Tema 1354. Aritmética por calendário civil.

## Reportar problemas
Abra uma issue usando o modelo "Relato de teste" — principalmente divergências
de cálculo, sempre com os dados de entrada (anonimizados) e o valor esperado.

*Uso interno e experimental. Não substitui a análise jurídica do caso.*
