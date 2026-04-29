Há quatro tipos de eventos de webhook de agente. Cada evento tem um valor enum numérico (usado nos payloads) e um nome canônico em string (usado no campo `event` do envelope e no cabeçalho HTTP `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | An agent run completes with status `SUCCESS`. |
| `trigger.failed` | 1 | An agent run completes with status `ERROR`. |
| `approval.requested` | 2 | An approval is queued in `PENDING` state. |
| `approval.decided` | 3 | An approval transitions to `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`. |

### `trigger.succeeded`

Disparado após a execução do agente terminar sem erro. O campo `data` do payload inclui:

- `triggerId` - o ID único da execução.
- `triggerType` - o [enum de motivo do trigger](#triggers-overview) que iniciou a execução.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokens consumidos nesta execução.
- `wasDryRun` - true se o agente estava em [modo dry-run](#dry-run-mode).
- `actions` - array de registros `TenantAgentAction` (veja [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - se o trigger os possuía.

Se a execução não realizou nenhuma ação, o array `actions` está vazio — esta é uma execução bem-sucedida em que "o agente decidiu não fazer nada", o que é útil saber.

### `trigger.failed`

Disparado quando uma execução encontra erro. Mesma estrutura de payload que `trigger.succeeded`, com `status: 'ERROR'` e um campo adicional `errorMessage` descrevendo o que deu errado. Erros possíveis incluem falhas em chamadas LLM, falhas no despacho de ferramentas e exaustão do orçamento no meio da execução.

`actions` ainda pode conter entradas para chamadas de ferramentas que foram concluídas antes do erro.

### `approval.requested`

Disparado no momento em que uma aprovação é enfileirada com estado `PENDING`. O payload inclui:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - os argumentos da ferramenta **passados literalmente** da chamada LLM. O formato é por-ferramenta e não é um contrato público estável - o esquema pode mudar conforme novas ferramentas são adicionadas.
- `createdAt`.
- `justification`, `confidence` - se o agente os forneceu.
- `contextSnapshot` - o contexto do comentário / página ao qual a aprovação se relaciona.

Útil para encaminhar aprovações pendentes para um canal de chat ops: um bot do Slack inscrito em `approval.requested` pode postar a ação e o raciocínio em um canal de moderação para revisão de relance.

### `approval.decided`

Disparado quando uma aprovação sai de `PENDING`. O payload inclui:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ou `EXECUTION_FAILED`.
- `decidedBy` - o ID do usuário moderador que tomou a decisão.
- `decidedAt` - quando ele decidiu.
- `executedAt` - se APPROVED, quando a plataforma executou a ação aprovada.
- `executionResult` - se APPROVED, uma string descrevendo o resultado do executor.
- `contextSnapshot` - o contexto do comentário / página.

Este evento cobre todos os resultados de decisão:

- **Aprovado + executado com sucesso** -> `status: APPROVED`, `executedAt` definido, `executionResult` é a mensagem de sucesso.
- **Aprovado + executor falhou** -> `status: EXECUTION_FAILED`, `executedAt` definido, `executionResult` descreve a falha.
- **Rejeitado** -> `status: REJECTED`, `executedAt` é null, `executionResult` é null.

### Header

Toda entrega inclui um cabeçalho HTTP `X-FastComments-Agent-Event` com o nome canônico em string do evento (`trigger.succeeded`, etc.). Útil se seu endpoint for uma única URL lidando com múltiplos tipos de evento.

### See also

- [Webhook Payloads](#webhook-payloads) para os esquemas completos de payload por evento.
- [Webhook Signing](#webhook-signing) para o esquema HMAC.
- [Webhook Retries](#webhook-retries) para a semântica de entrega.