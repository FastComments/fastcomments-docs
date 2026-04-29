Постоје четири типа агентских webhook догађаја. Сваки догађај има нумеричку enum вредност (која се користи у payload-овима) и канонско стринговско име (које се користи у пољу коверте `event` и у HTTP заглављу `X-FastComments-Agent-Event`).

| Назив догађаја | Enum | Покреће се када |
|---|---|---|
| `trigger.succeeded` | 0 | Извршавање агента се завршава са статусом `SUCCESS`. |
| `trigger.failed` | 1 | Извршавање агента се завршава са статусом `ERROR`. |
| `approval.requested` | 2 | Одобрење је стављено у ред у стању `PENDING`. |
| `approval.decided` | 3 | Одобрење прелази у `APPROVED`, `REJECTED` или `EXECUTION_FAILED`. |

### `trigger.succeeded`

Пали се након што се извршавање агента заврши без грешке. Поље `data` у payload-у садржи:

- `triggerId` - јединствени ID покретања.
- `triggerType` - [trigger reason enum](#triggers-overview) који је покренуо извршавање.
- `status` - `SUCCESS` (string).
- `tokensUsed` - токени утрошени у овом извршавању.
- `wasDryRun` - true ако је агент био у [dry-run mode](#dry-run-mode).
- `actions` - низ записа типа `TenantAgentAction` (видети [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ако су постојали у тригеру.

Ако извршавање није предузело ниједну акцију, низ `actions` је празан - ово је успешно „агент је одлучио да не ради ништа“ извршавање, што је корисно знати.

### `trigger.failed`

Пали се када извршавање баци грешку. Иста структура payload-а као за `trigger.succeeded`, са `status: 'ERROR'` и додатним пољем `errorMessage` које описује шта је пошло по злу. Могуће грешке укључују неуспехе позива LLM-а, неуспехе при покретању алата и истрошен буџет у току извршавања.

`actions` и даље може садржати уносе за позиве алата који су успели пре него што је дошло до грешке.

### `approval.requested`

Пали се у тренутку када је одобрење стављено у стање `PENDING`. Payload садржи:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - аргументи алата **пренети дословно** из позива LLM-а. Облик је по алату и није стабилан јавни уговор — шема се може променити како се додају нови алати.
- `createdAt`.
- `justification`, `confidence` - ако их је агент навео.
- `contextSnapshot` - снимак коментара / странице на који се одобрење односи.

Корисно за прослеђивање чекајућих одобрења у канал за chat ops: Slack бот претплаћен на `approval.requested` може да пошаље акцију и образложење у модерациони канал ради брзог прегледа.

### `approval.decided`

Пали се када одобрење изађе из стања `PENDING`. Payload садржи:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, или `EXECUTION_FAILED`.
- `decidedBy` - ID корисника модератора који је донео одлуку.
- `decidedAt` - када је донео одлуку.
- `executedAt` - ако је APPROVED, када је платформа извршила одобрено дејство.
- `executionResult` - ако је APPROVED, стринг који описује резултат извршитеља.
- `contextSnapshot` - снимак коментара / странице.

Овај догађај покрива све исходе одлуке:

- **Одобрено + извршено успешно** -> `status: APPROVED`, `executedAt` постављен, `executionResult` је порука о успеху.
- **Одобрено + извршитељ није успео** -> `status: EXECUTION_FAILED`, `executedAt` постављен, `executionResult` описује неуспех.
- **Одбијено** -> `status: REJECTED`, `executedAt` је null, `executionResult` је null.

### Заглавље

Свака достава садржи HTTP заглавље `X-FastComments-Agent-Event` са канонским стринговским именом догађаја (`trigger.succeeded`, итд.). Корисно ако ваш endpoint користи једну URL адресу за обраду више типова догађаја.

### Погледајте такође

- [Подаци webhook-а](#webhook-payloads) за потпune шеме payload-ова по догађају.
- [Потписивање webhook-а](#webhook-signing) за HMAC шему.
- [Поновљени покушаји webhook-а](#webhook-retries) за семантику доставе.

---