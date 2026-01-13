---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| userIdWS | string | Non |  |
| startTime | int64 | Non |  |
| endTime | int64 | Non |  |

## Réponse

Retourne: [`Option[GetEventLog_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getGlobalEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGlobalEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  userIdWS = "",
  startTime = int64(0),
  endTime = int64(0)
)
if response.isSome:
  let eventLog = response.get()
  echo eventLog
else:
  echo "No event log returned"
[inline-code-end]

---