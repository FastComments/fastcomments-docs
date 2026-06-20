req
tenantId
urlId
userIdWS

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| userIdWS | string | Ні |  |
| startTime | int64 | Ні |  |
| endTime | int64 | Ні |  |

## Відповідь

Повертає: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад використання getGlobalEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGlobalEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-2026-06-19",
  userIdWS = "user-987",
  startTime = int64(1622505600),
  endTime = int64(1625097600)
)
if response.isSome:
  let eventLog = response.get()
  echo eventLog, httpResponse.statusCode
[inline-code-end]