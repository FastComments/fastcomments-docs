Кожен webhook агента має власний журнал доставок. Доступний зі [сторінки зі списком webhook](https://fastcomments.com/auth/my-account/ai-agents/webhooks) через кнопку **Logs** у кожному рядку webhook.

### Що на сторінці

Таблиця з пагінацією з одним рядком на кожну спробу доставки:

| Column | Meaning |
|---|---|
| When | Коли сталася спроба. |
| Event | Тип події (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | Статус доставки. |
| StatusCode | HTTP status code returned by your endpoint, when available. |
| Description | Короткий опис результату. Для невдалих доставок, де HTTP-відповідь не була отримана, базове повідомлення про помилку зберігається як `{error: <message>}`. |

Сторінка підтримує лише пагінацію — немає фільтрів за статусом, типом події або діапазоном дат.

### Що ви можете зробити з журналів

- **Decide whether a status code should be in No-retry.** Якщо ви бачите, що ваш endpoint постійно повертає один і той же код `4xx`, це сильний сигнал, що ви хочете додати його до **No-retry status codes**, щоб платформа припинила повторні спроби.

### Інформація про помилки

Коли доставка не вдається без HTTP-відповіді (не вдалося вирішити DNS, з'єднання відхилено, таймаут, помилка TLS тощо), сире повідомлення про помилку записується як `{error: <message>}`. Платформа не класифікує ці помилки у іменовані категорії на кшталт `TIMEOUT` або `DNS_ERROR` — читайте повідомлення про помилку безпосередньо, щоб зрозуміти, що сталося.

Для HTTP-відповідей стовпець StatusCode показує код, повернений вашим endpoint. Типові випадки:

- **All attempts: `401` or `403`** - ваш endpoint відхиляє підпис. Перевірте, що ви обчислюєте HMAC над `${timestamp}.${body}` і використовуєте правильний секрет. Див. [Webhook Signing](#webhook-signing).
- **All attempts: `422`** - ваш endpoint вважає payload недійсним. Виправте endpoint або додайте `422` до **No-retry status codes**, якщо відхилення очікується для деяких подій.

### Контекст кожної доставки

Кожен запис журналу містить:

- `webhookId` - яка конфігурація webhook спричинила цю доставку.
- `agentId` - якому агенту стосується доставка.
- `triggerId` or `approvalId` - підлягаючий запис.
- `domain` - співпадаючий домен.

Ви можете використати ці поля, щоб зіставити невдалу доставку з виконанням, якому вона відповідає, у [Run History](#run-history).

### Термін зберігання

`AgentSyncLog` entries have a flat 1-year TTL on `createdAt` regardless of outcome - successful and failed deliveries are retained for the same length of time. Beyond retention, the log entry is gone.

Якщо вам потрібен довготривалий аудит, стійка практика така: нехай саме **endpoint** зберігає доставки, які він отримує. Це відокремлює ваш аудитний журнал від політики збереження платформи.

### Test send

Кнопка **Test send** у формі конфігурації webhook записує фейкову доставку в ту саму таблицю журналу, щоб ви могли перевірити підключення end-to-end перед тим, як покладатися на реальні події. Тестові доставки чітко позначені в журналі, щоб вони не спотворювали статистику невдач у продакшні.

### Див. також

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) for the retry semantics that drive these logs.
- [Webhook Signing](#webhook-signing) for how to verify on your endpoint.