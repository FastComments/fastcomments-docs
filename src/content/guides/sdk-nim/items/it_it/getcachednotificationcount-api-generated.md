## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |

## Risposta

Restituisce: [`Option[GetCachedNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getCachedNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "notif-thread-2026")
if response.isSome:
  let cached = response.get()
  echo "Cached notification count: ", $cached
[inline-code-end]