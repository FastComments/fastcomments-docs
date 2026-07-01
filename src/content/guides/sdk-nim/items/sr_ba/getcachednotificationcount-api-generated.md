## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Odgovor

Vraća: [`Option[GetCachedNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getCachedNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (cachedCountOpt, httpResp) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "article-456")
if cachedCountOpt.isSome:
  let cachedCount = cachedCountOpt.get()
  echo cachedCount
[inline-code-end]