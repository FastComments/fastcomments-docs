Има четири типа събития за webhook на агент. Всяко събитие има числова enum стойност (използвана в payload-ите) и канонично низово име (използвано в полето `event` на обвивката и в HTTP хедъра `X-FastComments-Agent-Event`).

| Event name | Enum | Кога се задейства |
|---|---|---|
| `trigger.succeeded` | 0 | Изпълнение на агент приключва със статус `SUCCESS`. |
| `trigger.failed` | 1 | Изпълнение на агент приключва със статус `ERROR`. |
| `approval.requested` | 2 | Заявка за одобрение е поставена в състояние `PENDING`. |
| `approval.decided` | 3 | Одобрение преминава в `APPROVED`, `REJECTED` или `EXECUTION_FAILED`. |

### `trigger.succeeded`

Задейства се след като изпълнението на агента приключи без грешка. Полето `data` в payload-а включва:

- `triggerId` - уникалният ID на изпълнението.
- `triggerType` - [enum на причините за тригър](#triggers-overview), който е стартирал изпълнението.
- `status` - `SUCCESS` (низ).
- `tokensUsed` - токените, потребени в това изпълнение.
- `wasDryRun` - true, ако агентът е бил в [режим dry-run](#dry-run-mode).
- `actions` - масив от записи `TenantAgentAction` (виж [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ако тригърът ги е имал.

Ако изпълнението е извършило нула действия, масивът `actions` е празен - това е успешно изпълнение тип "агентът реши да не прави нищо", което е полезно да се знае.

### `trigger.failed`

Задейства се когато изпълнението хване грешка. Същата форма на payload-a като при `trigger.succeeded`, с `status: 'ERROR'` и допълнително поле `errorMessage`, описващо какво се е объркало. Възможни грешки включват неуспехи при LLM повиквания, неуспехи при диспечериране на инструментите и изчерпване на бюджета по време на изпълнението.

`actions` все още може да съдържа записи за повиквания на инструменти, които са завършили преди грешката.

### `approval.requested`

Задейства се в момента, в който одобрение е поставено в състояние `PENDING`. Payload-ът включва:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - аргументите на инструмента, **предадени дословно** от повикването към LLM. Форматът е специфичен за всеки инструмент и не е стабилен публичен контракт - схемата може да се промени с добавянето на нови инструменти.
- `createdAt`.
- `justification`, `confidence` - ако агентът ги е предоставил.
- `contextSnapshot` - контекстът на коментара/страницата, към който се отнася одобрението.

Полезно за препращане на чакащи одобрения в канал за чат операции: Slack бот, абониран за `approval.requested`, може да публикува действието и мотивацията в модерационен канал за бърз преглед.

### `approval.decided`

Задейства се когато одобрение излезе от `PENDING`. Payload-ът включва:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, или `EXECUTION_FAILED`.
- `decidedBy` - ID на потребителя-модератор, който е взел решението.
- `decidedAt` - кога е взел решението.
- `executedAt` - ако е APPROVED, кога платформата е изпълнила одобреното действие.
- `executionResult` - ако е APPROVED, низ, описващ резултата от изпълнителя.
- `contextSnapshot` - контекстът на коментара/страницата.

Това събитие покрива всички възможни резултати от решението:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` е зададено, `executionResult` е съобщението за успешното изпълнение.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` е зададено, `executionResult` описва неизправността.
- **Rejected** -> `status: REJECTED`, `executedAt` е null, `executionResult` е null.

### Header

Всяка доставка включва HTTP хедър `X-FastComments-Agent-Event` със каноничното низово име на събитието (`trigger.succeeded`, и т.н.). Полезно, ако вашият endpoint е един и същ URL, който обработва множество типове събития.

### Виж също

- [Webhook полезни натоварвания](#webhook-payloads) за пълните схеми на полезните натоварвания за всяко събитие.
- [Подписване на Webhook](#webhook-signing) за HMAC схемата.
- [Повторни опити за Webhook](#webhook-retries) за семантиката на доставката.