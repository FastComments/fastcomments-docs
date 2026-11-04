Существует четыре типа событий вебхуков агента. Каждое событие имеет числовое значение enum (используется в payload) и каноническое строковое имя (используется в поле envelope `event` и в HTTP-заголовке `X-FastComments-Agent-Event`).

| Имя события | Enum | Когда срабатывает |
|---|---|---|
| `trigger.succeeded` | 0 | Запуск агента завершается со статусом `SUCCESS`. |
| `trigger.failed` | 1 | Запуск агента завершается со статусом `ERROR`. |
| `approval.requested` | 2 | Одобрение ставится в очередь в состоянии `PENDING`. |
| `approval.decided` | 3 | Одобрение переходит в состояние `APPROVED`, `REJECTED` или `EXECUTION_FAILED`. |

### `trigger.succeeded`

Срабатывает после завершения запуска агента без ошибок. Поле `data` payload включает:

- `triggerId` - уникальный ID запуска.
- `triggerType` - [enum причины запуска](#triggers-overview), который инициировал запуск.
- `status` - `SUCCESS` (строка).
- `tokensUsed` - токены, израсходованные в этом запуске.
- `wasDryRun` - true, если агент находился в [режиме dry-run](#dry-run-mode).
- `actions` - массив записей `TenantAgentAction` (см. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - если они были у триггера.

Если запуск не выполнил никаких действий, массив `actions` пуст - это успешный запуск «агент решил ничего не делать», что полезно знать.

### `trigger.failed`

Срабатывает при ошибке запуска. Та же структура payload, что и у `trigger.succeeded`, но со `status: 'ERROR'` и дополнительным полем `errorMessage`, описывающим, что пошло не так. Возможные ошибки включают сбои вызовов LLM, сбои диспетчеризации инструментов и исчерпание бюджета посреди запуска.

`actions` всё ещё может содержать записи о вызовах инструментов, завершившихся до ошибки.

### `approval.requested`

Срабатывает в момент, когда одобрение ставится в очередь в состоянии `PENDING`. Payload включает:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - аргументы инструмента, **переданные дословно** из вызова LLM. Структура зависит от инструмента и не является стабильным публичным контрактом - схема может меняться по мере добавления новых инструментов.
- `createdAt`.
- `justification`, `confidence` - если агент их предоставил.
- `contextSnapshot` - контекст комментария / страницы, к которому относится одобрение.

Полезно для пересылки ожидающих одобрений в канал chat ops: Slack-бот, подписанный на `approval.requested`, может публиковать действие и обоснование в канал модерации для быстрого просмотра.

### `approval.decided`

Срабатывает, когда одобрение выходит из состояния `PENDING`. Payload включает:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` или `EXECUTION_FAILED`.
- `decidedBy` - ID пользователя-модератора, принявшего решение.
- `decidedAt` - когда было принято решение.
- `executedAt` - если APPROVED, когда платформа выполнила одобренное действие.
- `executionResult` - если APPROVED, строка, описывающая результат выполнения.
- `contextSnapshot` - контекст комментария / страницы.

Это событие охватывает все исходы решений:

- **Одобрено и выполнено успешно** -> `status: APPROVED`, `executedAt` установлен, `executionResult` содержит сообщение об успехе.
- **Одобрено, но выполнение не удалось** -> `status: EXECUTION_FAILED`, `executedAt` установлен, `executionResult` описывает сбой.
- **Отклонено** -> `status: REJECTED`, `executedAt` равен null, `executionResult` равен null.

### Заголовок

Каждая доставка включает HTTP-заголовок `X-FastComments-Agent-Event` с каноническим строковым именем события (`trigger.succeeded` и т.д.). Полезно, если ваш эндпоинт — это один URL, обрабатывающий несколько типов событий.

### См. также

- [Webhook Payloads](#webhook-payloads) - полные схемы payload для каждого события.
- [Webhook Signing](#webhook-signing) - схема HMAC.
- [Webhook Retries](#webhook-retries) - семантика доставки.