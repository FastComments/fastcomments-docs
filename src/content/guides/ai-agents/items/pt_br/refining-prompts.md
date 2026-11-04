**Refine Prompt** (Refinar Prompt) é o fluxo de trabalho para editar o [prompt inicial](#personality-prompt) de um agente em resposta a decisões específicas com as quais você discorda. Ele é iniciado a partir da [caixa de entrada de aprovações](#approval-workflow).

### Quando usar

Quando você se pega rejeitando o mesmo tipo de aprovação repetidamente - "o agente continua querendo banir pessoas por usar linguagem forte sem um alvo" - o prompt do agente é a alavanca para corrigir isso. O Refine Prompt é uma forma guiada de:

1. Escolher uma aprovação específica que represente a decisão ruim.
2. Editar o prompt com o contexto completo do que o agente fez e por quê.
3. Salvar o novo prompt no agente.

O resultado é um agente que, de agora em diante, dificilmente tomaria a mesma decisão.

### Iniciando o fluxo

A partir da caixa de entrada de aprovações em `/auth/my-account/ai-agent-approvals`:

1. Abra uma aprovação **rejeitada**. A rota rejeita imediatamente qualquer coisa exceto REJECTED - aprovações pendentes e com falha na execução não são elegíveis.
2. Clique em **Refine prompt**.

Você chega na interface de refinamento de prompt em `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### O que a página mostra

- **A aprovação** - o `toolName` e a `justification` do agente para a decisão rejeitada (a transcrição completa do LLM não é exibida aqui).
- **O prompt atual** - o [prompt inicial](#personality-prompt) salvo do agente.
- **Um campo de feedback** - você digita o **feedback** descrevendo o que deve mudar (até 2000 caracteres). O LLM então gera o novo prompt proposto a partir do seu feedback.
- **Diff inline unificado** - um único diff inline entre o prompt atual e o proposto (vermelho para removido, verde para adicionado).

O contexto da aprovação fica fixado no topo para que você possa continuar consultando "o caso que estou corrigindo" enquanto edita.

### Salvar

Salvar atualiza o campo `initialPrompt` do agente. Execuções passadas (e aprovações passadas) não são reexecutadas retroativamente - o novo prompt afeta apenas gatilhos futuros. Se você quiser verificar se o novo prompt corrige o problema, execute um [test run / replay](#test-runs-replays) contra os últimos 7 dias e veja se o novo prompt ainda produziria a aprovação rejeitada.

### O que o fluxo não faz

- Não edita as **diretrizes da comunidade** - esse campo tem seu próprio editor no formulário principal de edição do agente.
- Não edita **gatilhos**, **ferramentas permitidas** ou **controle de aprovação** - esses permanecem no formulário principal de edição.
- Não versiona o prompt com rollback. O prompt anterior não é armazenado em uma coleção separada de histórico. Se você precisar reverter, copie o prompt atual para seu próprio sistema de controle antes de editar.

### Por que combinar refine com replay

Editar um prompt sem testar o resultado é baseado na fé. O ciclo recomendado:

1. Rejeite uma aprovação.
2. Refine o prompt.
3. Execute um [test run](#test-runs-replays) contra os últimos 7 dias.
4. Olhe a aba **Deltas**. O novo prompt moveu a decisão ruim de "faria" para "não faria"? Ele acidentalmente moveu também boas decisões?
5. Itere.

Três ou quatro ciclos de refine + replay geralmente são suficientes para obter um prompt estável para um agente de moderação.

### Alternativa de edição direta

Você não precisa usar o Refine Prompt - você também pode simplesmente editar o agente no formulário principal de edição. A única vantagem do Refine Prompt é que ele fixa um caso específico de falha para que você não perca o foco do que está corrigindo.