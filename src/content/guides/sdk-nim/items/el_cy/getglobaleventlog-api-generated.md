req
tenantId
urlId
userIdWS

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| userIdWS | string | Όχι |  |
| startTime | int64 | Όχι |  |
| endTime | int64 | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGlobalEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---