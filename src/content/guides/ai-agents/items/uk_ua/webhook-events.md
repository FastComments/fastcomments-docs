Є чотири типи подій агентських webhook. Кожна подія має числове значення enum (використовується в payload) та канонічну стрічкову назву (використовується в полі оболонки `event` та в HTTP-заголовку `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Прохід агента завершується зі статусом `SUCCESS`. |
| `trigger.failed` | 1 | Прохід агента завершується зі статусом `ERROR`. |
| `approval.requested` | 2 | Затвердження поставлено в чергу в стані `PENDING`. |
| `approval.decided` | 3 | Затвердження переходить у `APPROVED`, `REJECTED`, або `EXECUTION_FAILED`. |

### `trigger.succeeded`

Відбувається після того, як прогін агента завершується без помилок. Поле `data` в payload містить:

- `triggerId` - унікальний ID запуску.
- `triggerType` - [trigger reason enum](#triggers-overview), який запустив прогін.
- `status` - `SUCCESS` (рядок).
- `tokensUsed` - кількість токенів, витрачених у цьому прогоні.
- `wasDryRun` - true, якщо агент був у [dry-run mode](#dry-run-mode).
- `actions` - масив записів `TenantAgentAction` (див. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - якщо вони були в триґері.

Якщо прогін не виконав жодної дії, масив `actions` порожній — це успішний прогін «the agent decided to do nothing», що корисно знати.

### `trigger.failed`

Відбувається, коли прогін завершується помилкою. Такий же формат payload, як у `trigger.succeeded`, з `status: 'ERROR'` та додатковим полем `errorMessage`, яке описує, що пішло не так. Можливі помилки включають відмови викликів LLM, відмови відправлення запитів до інструментів та вичерпання бюджету посеред прогону.

Масив `actions` все ще може містити записи для викликів інструментів, які завершилися до помилки.

### `approval.requested`

Відбувається в момент, коли затвердження ставиться в чергу в стані `PENDING`. Payload містить:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - аргументи інструмента, **передані дослівно** (passed through verbatim) з виклику LLM. Форма визначається для кожного інструмента і не є стабільним публічним контрактом — схема може змінюватися у міру додавання нових інструментів.
- `createdAt`.
- `justification`, `confidence` - якщо агент їх надав.
- `contextSnapshot` - контекст коментаря/сторінки, до якого відноситься затвердження.

Корисно для пересилання очікуваних затверджень у чат-операційний канал: Slack-бот, підписаний на `approval.requested`, може опублікувати дію та аргументацію в канал модерування для швидкого перегляду.

### `approval.decided`

Відбувається, коли затвердження виходить із стану `PENDING`. Payload містить:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, або `EXECUTION_FAILED`.
- `decidedBy` - ID користувача-модератора, який ухвалив рішення.
- `decidedAt` - коли було ухвалено рішення.
- `executedAt` - якщо APPROVED, коли платформа виконала затверджену дію.
- `executionResult` - якщо APPROVED, рядок, що описує результат виконавця.
- `contextSnapshot` - контекст коментаря/сторінки.

Ця подія охоплює всі результати ухвалення:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` встановлено, `executionResult` — повідомлення про успіх.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` встановлено, `executionResult` описує відмову.
- **Rejected** -> `status: REJECTED`, `executedAt` дорівнює null, `executionResult` дорівнює null.

### Заголовок

Кожна доставка включає HTTP-заголовок `X-FastComments-Agent-Event` з канонічною стрічковою назвою події (`trigger.succeeded`, тощо). Це корисно, якщо ваш endpoint — одна URL-адреса, що обробляє кілька типів подій.

### Див. також

- [Webhook Payloads](#webhook-payloads) для повних схем payload по подіях.
- [Webhook Signing](#webhook-signing) для схеми HMAC.
- [Webhook Retries](#webhook-retries) для семантики доставки.