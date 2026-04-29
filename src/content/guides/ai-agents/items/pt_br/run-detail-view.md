Clicar em **Visualizar** em uma linha em [Histórico de Execuções](#run-history) abre a página de detalhes por execução. É aqui que você lê o raciocínio do agente e avalia suas decisões.

### Topo: resumo da execução

- **Agent** - qual agente executou.
- **When** - timestamp.
- **Status** - Started / Success / Error, além do selo **Execução simulada** quando aplicável.
- **Cost** - custo por execução na moeda do seu tenant.
- **Cost per action** - custo dividido pela contagem de ações não pendentes, útil para identificar execuções incomumente caras.

### Ações executadas

Uma lista, em ordem, de cada chamada de ferramenta que a execução realizou. Cada entrada mostra:

- **Action label** - "Wrote a comment", "Marked a comment as spam", "Banned a user", e assim por diante. O rótulo mapeia a partir do enum de tipos de ação.
- **Reference ID** - o ID do comentário, usuário ou badge afetado, mostrado como texto monoespaçado (não um hyperlink).
- **Agent reasoning** - a justificativa que o agente forneceu com a chamada.
- **Confidence** - a autoconfiança do agente, exibida como porcentagem.
- **Pending approval** badge - se a ação estiver enfileirada na [caixa de entrada de aprovações](#approval-workflow) ao invés de executada.

Se a execução não realizou nenhuma ação, a seção diz: "No actions were taken during this run."

### Transcrição do LLM

Abaixo das ações, a transcrição completa da conversa do agente com o LLM:

- **System** - o system prompt (sufixo da plataforma + seu prompt inicial + diretrizes da comunidade).
- **User** - a mensagem de contexto descrevendo o gatilho.
- **Assistant** - as respostas do modelo, incluindo chamadas de ferramenta.
- **Tool** - o resultado da ferramenta alimentado de volta ao modelo (por exemplo, o que `search_memory` retornou).

Mensagens longas são recolhíveis; clique **Expandir** / **Recolher** para visualizar.

### Lendo transcrições

A transcrição é a página mais importante para ajuste fino. Quando o agente toma uma decisão com a qual você discorda, leia-a para saber:

- O que o modelo **viu** (a mensagem de contexto do User).
- O que o modelo **decidiu** (as chamadas de ferramenta do Assistant).
- O que o modelo **considerou** (quaisquer resultados de ferramenta - por exemplo, o agente realmente chamou `search_memory` e encontrou algo antes de banir).

Se o modelo estiver cometendo consistentemente o mesmo tipo de erro, edite o [prompt inicial](#personality-prompt) - ou use [Refinar Prompts](#refining-prompts) a partir de uma aprovação rejeitada.

### Referências de ação

Os IDs de referência são mostrados como texto monoespaçado (não hyperlinks):

- Comentários: o ID do comentário.
- Usuários: o ID do usuário.
- Badges: o ID do badge.

Você pode copiar o ID para procurar o registro afetado na página de moderação/administrativa relevante.

### O que está faltando em execução simulada

Execuções simuladas mostram as mesmas ações, justificativas e pontuações de confiança. A única diferença é o selo **Execução simulada** na linha de status. Os IDs de referência para comentários / usuários / badges ainda são mostrados - o agente apenas não os afetou.

### Erros

Para execuções em `Error` state, a página de detalhes mostra a mensagem de erro subjacente. Erros comuns:

- **No LLM API key configured** - misconfiguração do tenant ou da plataforma.
- **LLM call timeout** - o provedor de LLM estava lento ou indisponível.
- **Tool dispatch failure** - o agente escolheu uma ferramenta com argumentos inválidos (por exemplo, um ID de comentário que não existe mais).
- **Budget exhausted mid-run** - o limite do agente foi atingido enquanto a execução estava em andamento. A execução foi interrompida.

Erros não desfazem ações parciais - quaisquer chamadas de ferramenta concluídas antes do erro permanecem.