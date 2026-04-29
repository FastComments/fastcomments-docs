**Dry Run** é o modo de segurança em que todo novo agente é iniciado. O agente executa o fluxo de ponta a ponta, exceto na parte em que interage com sua comunidade.

### What runs in Dry Run

- Os gatilhos disparam normalmente.
- O prompt do agente, as [diretrizes da comunidade](#community-guidelines), e o [contexto](#context-options) são montados.
- The LLM is called.
- O modelo seleciona chamadas de ferramentas e fornece justificativas + pontuações de confiança.
- A execução é registrada com um **Dry Run** badge para que fique claramente diferenciada das execuções ao vivo.

### What does not run in Dry Run

- Nenhum comentário é publicado, nenhum voto é computado, nenhum comentário é fixado/desfixado/bloqueado/desbloqueado.
- Nenhum comentário é marcado como spam, aprovado ou revisado.
- Nenhum usuário é banido, advertido ou recebe um distintivo.
- Nenhum e-mail é enviado.
- Nenhuma memória é gravada. (Sim — incluindo a memória. Agentes em Dry Run não podem contaminar o pool de memória compartilhada com decisões hipotéticas.)
- Nenhum webhook é disparado para ações de ferramentas. (Os webhooks em nível de trigger `trigger.succeeded` / `trigger.failed` ainda são acionados e o payload inclui `wasDryRun: true`. Veja [Webhook Payloads](#webhook-payloads).)

### What it costs

Executions em Dry Run utilizam **a mesma LLM call** que uma execução Enabled usaria. Tokens são cobrados, os [limites de orçamento](#budgets-overview) se aplicam, e as execuções contam contra os limites diários/mensais por agente e por tenant.

Esse custo é o preço por obter uma prévia fiel. Um modo de "pular a chamada LLM" não lhe daria nenhum indício de como o agente se comportaria.

### Reading dry-run results

Em [Run History](#run-history), execuções em dry-run são marcadas com o distintivo **Dry Run** na coluna de status. As ações dentro de cada execução parecem idênticas às ações ao vivo — mesmo nome da ferramenta, mesmos argumentos, mesma justificativa e confiança — exceto que nenhuma delas aconteceu.

A [Analytics page](#analytics-page) detalha as execuções "dry-run vs live" por mês para que você possa ver quanto do seu consumo de tokens foi gasto com observação.

### Switching out of Dry Run

Edit the agent and change **Status** to **Enabled**. O próximo trigger será executado ao vivo.

Você também pode fazer o caminho inverso — Enabled de volta para Dry Run — se o agente começar a fazer coisas que você não goste. Não há penalidade.

### Replays force Dry Run

O recurso [Test Runs (Replays)](#test-runs-replays) executa o agente contra comentários históricos **sempre em dry-run**, independentemente do status salvo do agente. Replays não podem realizar ações reais em comentários passados. Isso é proposital — replay é uma ferramenta de pré-visualização, não uma ferramenta de moderação.

---