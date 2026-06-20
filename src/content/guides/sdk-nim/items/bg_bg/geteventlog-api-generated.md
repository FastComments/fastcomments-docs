req
tenantId
urlId
userIdWS

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| userIdWS | string | Не |  |
| startTime | int64 | Не |  |
| endTime | int64 | Не |  |

## Отговор

Връща: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-2026-solar-panels",
  userIdWS = "user-456",
  startTime = 1688000000'i64,
  endTime = 1688086400'i64
)
if response.isSome:
  let eventLog = response.get()
  discard eventLog
[inline-code-end]

---