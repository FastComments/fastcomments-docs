Постоје четири типа агенских webhook догађаја. Сваки догађај има нумеричку вредност енум-а (која се користи у payload-овима) и канонско стринг име (које се користи у пољу `event` омотача и у HTTP заглављу `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Покреће се када агенсов покрет заврши са статусом `SUCCESS`. |
| `trigger.failed` | 1 | Покреће се када агенсов покрет заврши са статусом `ERROR`. |
| `approval.requested` | 2 | Покреће се када је одобрење стављено у ред у стање `PENDING`. |
| `approval.decided` | 3 | Покреће се када одобрење пређе у `APPROVED`, `REJECTED`, или `EXECUTION_FAILED`. |

### `trigger.succeeded`

Покреће се након што агенсов покрет заврши без грешке. Поље `data` у payload-у укључује:

- `triggerId` - јединствени ID покрета.
- `triggerType` - [trigger reason enum](#triggers-overview) који је покренуо извршавање.
- `status` - `SUCCESS` (стринг).
- `tokensUsed` - токени потрошени у овом покрету.
- `wasDryRun` - true ако је агент био у [dry-run режиме](#dry-run-mode).
- `actions` - низ `TenantAgentAction` записа (погледајте [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ако је тригер имао ове податке.

Ако је покрет обавио нула акција, низ `actions` је празан — ово је успешан покрет где је агент одлучио да не уради ништа, што је корисна информација.

### `trigger.failed`

Покреће се када покрет заврши са грешком. Исто обликовање payload-а као за `trigger.succeeded`, са `status: 'ERROR'` и додатним пољем `errorMessage` које описује шта је поишло по злу. Могуће грешке укључују неуспехе позива LLM-а, неуспехе покретања алата и истрошен буџет током извршавања.

`actions` и даље може да садржи уносе за позиве алата који су се завршили пре грешке.

### `approval.requested`

Покреће се у тренутку када је одобрење стављено у ред у стање `PENDING`. Payload укључује:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - аргументи алата **прослеђени непромењени** из позива LLM-а. Облик је специфичан по алату и није стабилан јавни уговор - шема може да се промени како се додају нови алати.
- `createdAt`.
- `justification`, `confidence` - ако их је агент навео.
- `contextSnapshot` - снимак коментара / странице на који се одобрење односи.

Корисно за прослеђивање очекујућих одобрења у чат-оперативни канал: Slack бот који је претплаћен на `approval.requested` може објавити акцију и образложење у каналу за модерацију за брзи преглед.

### `approval.decided`

Покреће се када одобрење изађе из стања `PENDING`. Payload укључује:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, или `EXECUTION_FAILED`.
- `decidedBy` - ID корисника модератора који је донео одлуку.
- `decidedAt` - када је одлучио.
- `executedAt` - ако је APPROVED, када платформа извршила одобрена радњу.
- `executionResult` - ако је APPROVED, стринг који описује резултат извршилаца.
- `contextSnapshot` - снимак коментара / странице.

Овај догађај покрива све исходе одлуке:

- **Одобрено + извршено успешно** -> `status: APPROVED`, `executedAt` подешен, `executionResult` је порука о успеху.
- **Одобрено + извршилац није успео** -> `status: EXECUTION_FAILED`, `executedAt` подешен, `executionResult` описује неуспех.
- **Одбијено** -> `status: REJECTED`, `executedAt` је null, `executionResult` је null.

### Header

Свака испорука садржи HTTP заглавље `X-FastComments-Agent-Event` са канонским стринг именом догађаја (`trigger.succeeded`, итд.). Корисно ако ваш крајњи URL обрађује више типова догађаја.

### See also

- [Webhook Payloads](#webhook-payloads) за пуну схему payload-ова по догађају.
- [Webhook Signing](#webhook-signing) за HMAC шему.
- [Webhook Retries](#webhook-retries) за семантику испоруке.