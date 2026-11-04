**Dry Run** é o modo de segurança em que todo novo agente começa. O agente roda de ponta a ponta, exceto pela parte em que ele toca na sua comunidade.

### O que roda no Dry Run

- Os gatilhos disparam normalmente.
- O prompt do agente, as [diretrizes da comunidade](#community-guidelines) e o [contexto](#context-options) são montados.
- O LLM é chamado.
- O modelo escolhe as chamadas de ferramenta e fornece justificativas + pontuações de confiança.
- A execução é registrada com um selo **Dry Run** para que seja claramente distinguida das execuções reais.

### O que não roda no Dry Run

- Nenhum comentário é postado, nenhum voto é dado, nenhum comentário é fixado/desfixado/bloqueado/desbloqueado.
- Nenhum comentário é marcado como spam, aprovado ou revisado.
- Nenhum usuário é banido, advertido ou recebe um selo.
- Nenhum e-mail é enviado.
- Nenhuma memória é gravada. (Sim - incluindo memória. Agentes em dry-run não podem envenenar o pool de memória compartilhada com decisões hipotéticas.)
- Nenhum webhook dispara para ações de ferramentas. (Os webhooks de nível de gatilho `trigger.succeeded` / `trigger.failed` ainda disparam e o payload inclui `wasDryRun: true`. Veja [Payloads de Webhook](#webhook-payloads).)

### Quanto custa

O Dry Run faz **a mesma chamada de LLM** que uma execução habilitada faria. Tokens são cobrados, os [limites de orçamento](#budgets-overview) se aplicam, e as execuções contam para os limites diários/mensais por agente e por tenant.

Esse custo é o preço de obter uma prévia fiel. Um modo de "pular a chamada do LLM" não daria nenhum sinal sobre como o agente se comportaria.

### Lendo os resultados do dry-run

Em [Histórico de Execuções](#run-history), as execuções em dry-run são marcadas com o selo **Dry Run** na coluna de status. As ações dentro de cada execução parecem idênticas às ações reais - mesmo nome de ferramenta, mesmos argumentos, mesma justificativa e confiança - exceto que nenhuma delas aconteceu.

A [página de Analytics](#analytics-page) detalha as execuções "dry-run vs live" por mês para que você possa ver quanto do seu gasto com tokens foi destinado à observação.

### Saindo do Dry Run

Edite o agente e altere o **Status** para **Enabled**. O próximo gatilho executará de verdade.

Você também pode mudar no sentido contrário - de Enabled de volta para Dry Run - se o agente começar a fazer coisas que você não gosta. Não há penalidade.

### Replays forçam o Dry Run

O recurso [Execuções de Teste (Replays)](#test-runs-replays) roda o agente contra comentários históricos **sempre em dry-run**, independentemente do status salvo do agente. Replays não podem tomar ações reais em comentários passados. Isso é intencional - o replay é uma ferramenta de prévia, não uma ferramenta de moderação.