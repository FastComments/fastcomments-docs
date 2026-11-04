Existem quatro tipos de eventos de webhook de agente. Cada evento tem um valor enum numérico (usado nos payloads) e um nome canônico em string (usado no campo `event` do envelope e no header HTTP `X-FastComments-Agent-Event`).

| Nome do evento | Enum | Disparado quando |
|---|---|---|
| `trigger.succeeded` | 0 | Uma execução do agente é concluída com status `SUCCESS`. |
| `trigger.failed` | 1 | Uma execução do agente é concluída com status `ERROR`. |
| `approval.requested` | 2 | Uma aprovação é enfileirada no estado `PENDING`. |
| `approval.decided` | 3 | Uma aprovação transita para `APPROVED`, `REJECTED` ou `EXECUTION_FAILED`. |

### `trigger.succeeded`

Disparado após a execução do agente terminar sem erro. O campo `data` do payload inclui:

- `triggerId` - o ID único da execução.
- `triggerType` - o [enum de motivo do gatilho](#triggers-overview) que iniciou a execução.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokens consumidos nesta execução.
- `wasDryRun` - true se o agente estava em [modo dry-run](#dry-run-mode).
- `actions` - array de registros `TenantAgentAction` (veja [Payloads de Webhook](#webhook-payloads)).
- `commentId`, `url`, `urlId` - se o gatilho os tinha.

Se a execução não realizou nenhuma ação, o array `actions` estará vazio - esta é uma execução bem-sucedida do tipo "o agente decidiu não fazer nada", o que é útil saber.

### `trigger.failed`

Disparado quando uma execução falha. Mesmo formato de payload que `trigger.succeeded`, com `status: 'ERROR'` e um campo adicional `errorMessage` descrevendo o que deu errado. Erros possíveis incluem falhas em chamadas de LLM, falhas no despacho de ferramentas e esgotamento do orçamento no meio da execução.

`actions` ainda pode conter entradas para chamadas de ferramentas que foram concluídas antes do erro.

### `approval.requested`

Disparado no momento em que uma aprovação é enfileirada no estado `PENDING`. O payload inclui:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - os argumentos da ferramenta **passados verbatim** da chamada de LLM. O formato é específico por ferramenta e não é um contrato público estável - o schema pode mudar conforme novas ferramentas são adicionadas.
- `createdAt`.
- `justification`, `confidence` - se o agente os forneceu.
- `contextSnapshot` - o contexto do comentário / página ao qual a aprovação se refere.

Útil para encaminhar aprovações pendentes para um canal de chat ops: um bot do Slack inscrito em `approval.requested` pode publicar a ação e o raciocínio em um canal de moderação para revisão rápida.

### `approval.decided`

Disparado quando uma aprovação sai do estado `PENDING`. O payload inclui:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` ou `EXECUTION_FAILED`.
- `decidedBy` - o ID do usuário do moderador que decidiu.
- `decidedAt` - quando ele decidiu.
- `executedAt` - se APPROVED, quando a plataforma executou a ação aprovada.
- `executionResult` - se APPROVED, uma string descrevendo o resultado do executor.
- `contextSnapshot` - o contexto do comentário / página.

Este evento cobre todos os resultados de decisão:

- **Aprovado + executado sem problemas** -> `status: APPROVED`, `executedAt` definido, `executionResult` é a mensagem de sucesso.
- **Aprovado + falha do executor** -> `status: EXECUTION_FAILED`, `executedAt` definido, `executionResult` descreve a falha.
- **Rejeitado** -> `status: REJECTED`, `executedAt` é null, `executionResult` é null.

### Header

Cada entrega inclui um header HTTP `X-FastComments-Agent-Event` com o nome canônico em string do evento (`trigger.succeeded`, etc.). Útil se seu endpoint é uma única URL que trata múltiplos tipos de eventos.

### Veja também

- [Payloads de Webhook](#webhook-payloads) para os schemas completos de payload por evento.
- [Assinatura de Webhook](#webhook-signing) para o esquema HMAC.
- [Retries de Webhook](#webhook-retries) para a semântica de entrega.