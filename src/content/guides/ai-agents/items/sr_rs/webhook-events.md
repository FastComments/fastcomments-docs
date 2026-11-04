Постоје четири типа agent webhook догађаја. Сваки догађај има нумеричку enum вредност (користи се у payload-овима) и канонско стринг име (користи се у `event` envelope пољу и у `X-FastComments-Agent-Event` HTTP заглављу).

| Име догађаја | Enum | Активира се када |
|---|---|---|
| `trigger.succeeded` | 0 | Agent run се завршава са статусом `SUCCESS`. |
| `trigger.failed` | 1 | Agent run се завршава са статусом `ERROR`. |
| `approval.requested` | 2 | Одобрење се ставља у ред у `PENDING` стању. |
| `approval.decided` | 3 | Одобрење прелази у `APPROVED`, `REJECTED` или `EXECUTION_FAILED`. |

### `trigger.succeeded`

Активира се када се agent run заврши без грешке. Поље `data` у payload-у садржи:

- `triggerId` - јединствени ID извршавања.
- `triggerType` - [enum разлога покретања](#triggers-overview) који је покренуо извршавање.
- `status` - `SUCCESS` (стринг).
- `tokensUsed` - токени потрошени у овом извршавању.
- `wasDryRun` - тачно ако је agent био у [dry-run режиму](#dry-run-mode).
- `actions` - низ `TenantAgentAction` записа (види [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ако их је trigger имао.

Ако извршавање није предузело ниједну акцију, низ `actions` је празан - то је успешно извршавање типа "agent је одлучио да не ради ништа", што је корисно знати.

### `trigger.failed`

Активира се када извршавање наиђе на грешку. Исти облик payload-а као `trigger.succeeded`, са `status: 'ERROR'` и додатним пољем `errorMessage` које описује шта је пошло наопако. Могуће грешке укључују неуспехе LLM позива, неуспехе dispatch-а алата и исцрпљивање буџета током извршавања.

`actions` и даље може садржати уносе за позиве алата који су завршени пре грешке.

### `approval.requested`

Активира се у тренутку када се одобрење стави у ред у `PENDING` стању. Payload садржи:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - аргументи алата **прослеђени дословно** из LLM позива. Облик зависи од алата и није стабилан јавни уговор - шема се може мењати како се додају нови алати.
- `createdAt`.
- `justification`, `confidence` - ако их је agent доставио.
- `contextSnapshot` - контекст коментара / странице на који се одобрење односи.

Корисно за прослеђивање одобрења на чекању у chat ops канал: Slack бот претплаћен на `approval.requested` може објавити акцију и образложење у модерациони канал за преглед једним погледом.

### `approval.decided`

Активира се када одобрење изађе из `PENDING` стања. Payload садржи:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` или `EXECUTION_FAILED`.
- `decidedBy` - ID корисника модератора који је донео одлуку.
- `decidedAt` - када је одлука донета.
- `executedAt` - ако је APPROVED, када је платформа извршила одобрену акцију.
- `executionResult` - ако је APPROVED, стринг који описује резултат извршиоца.
- `contextSnapshot` - контекст коментара / странице.

Овај догађај покрива све исходе одлука:

- **Одобрено + успешно извршено** -> `status: APPROVED`, `executedAt` постављено, `executionResult` је порука о успеху.
- **Одобрено + извршилац није успео** -> `status: EXECUTION_FAILED`, `executedAt` постављено, `executionResult` описује неуспех.
- **Одбијено** -> `status: REJECTED`, `executedAt` је null, `executionResult` је null.

### Заглавље

Свака испорука укључује HTTP заглавље `X-FastComments-Agent-Event` са канонским стринг именом догађаја (`trigger.succeeded`, итд.). Корисно ако је ваш endpoint један URL који рукује више типова догађаја.

### Погледајте такође

- [Webhook Payloads](#webhook-payloads) за пуне шеме payload-а по догађају.
- [Webhook Signing](#webhook-signing) за HMAC шему.
- [Webhook Retries](#webhook-retries) за семантику испоруке.