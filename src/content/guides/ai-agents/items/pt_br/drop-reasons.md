Quando um gatilho é acionado para um agente mas **não** resulta em uma chamada LLM, a plataforma registra um "drop" com um motivo. Drops aparecem na [Página de Analytics](#analytics-page) em "Gatilhos ignorados (este mês)".

### A lista completa de motivos de drop

| Reason | What happened |
|---|---|
| `agentDaily` | The agent's daily budget cap was hit. |
| `agentMonthly` | The agent's monthly budget cap was hit. |
| `tenantDaily` | The tenant's daily budget cap was hit. |
| `tenantMonthly` | The tenant's monthly budget cap was hit. |
| `qps` | The agent's per-minute rate limit (rolling 60s window) was hit. |
| `concurrency` | The agent's max concurrent runs was already saturated. |

### O que não está nesta lista

Um gatilho que nunca alcança o caminho de despacho não é "dropped" com um motivo - ele simplesmente não é despachado. Isso inclui:

- O agente está **Desativado**.
- O comentário que acionou não corresponde ao [Escopo de URL/locale](#scope-url-locale) do agente.
- A ação que acionou foi feita pelo mesmo agente (prevenção de loop).
- O tenant tem cobrança inválida.
- O agente não está no plano do tenant.

Essas são omissões silenciosas, não drops. Elas não aparecem no gráfico de drops no Analytics.

### Visualizando drops no Analytics

A [Página de Analytics](#analytics-page) mostra:

- **Triggers skipped (this month)** - contagens agrupadas por motivo de drop.
- **Agents at or near their cap** - detalhamento por agente de quais agentes estão atingindo o limite, com uma contagem de gatilhos descartados no período atual.

### O que fazer quando você vê drops

- **`agentDaily` / `agentMonthly`** - o limite do próprio agente está muito apertado. Aumente o limite no formulário de edição ou reduza o escopo do agente (URL/locale, gatilhos mais restritos).
- **`tenantDaily` / `tenantMonthly`** - o limite a nível de conta está muito apertado. Aumente-o nas configurações de cobrança do tenant, ou distribua o gasto entre menos agentes.
- **`qps`** - o tráfego está atingindo o limite por minuto na janela rolante. Frequentemente um sinal de um thread viral que dispara gatilhos mais rápido do que o agente consegue executá-los. Os campos `maxTriggersPerMinute` e `maxConcurrent` do agente limitam isso; aumentá-los eleva a taxa de processamento, mas também aumenta o custo de picos.
- **`concurrency`** - mesma causa raiz que `qps`, mas no número de execuções em voo. Aumente `maxConcurrent` se você precisar de mais paralelismo.

### Drops vs erros

Um drop é "o gatilho nunca executou". Um **erro** é "o gatilho executou, mas a chamada LLM ou o despacho de ferramenta falhou". Erros são rastreados separadamente na página [Run History](#run-history) (status `Error`).

### Drops também podem parar replays

Os mesmos motivos de drop interrompem [test runs / replays](#test-runs-replays) em andamento. O replay para com status Error e uma mensagem que informa qual orçamento foi atingido (por exemplo, o orçamento diário do agente).

### A prevenção de loop é silenciosa de propósito

Não existe um motivo de drop para "este gatilho veio de outro agente e foi pulado para prevenir um loop". Registrá-lo poluiria o Analytics sem sinal útil — por design, o fan-out de agentes nunca deve resultar em explosão de gatilhos. Se você suspeita que um loop está sendo suprimido onde não deveria, verifique os [Logs de comentários](/guide-moderation.html#comment-logs) - o `botId` em comentários escritos pelo bot é o que a verificação de loop utiliza.