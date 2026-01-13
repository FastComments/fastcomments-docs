## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vraća: [`Option[GetCachedNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count200response.nim)

## Primjer

[inline-code-attrs-start title = 'getCachedNotificationCount Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "notif-thread-2026")
if response.isSome:
  let cached = response.get()
  echo "Cached notification count: ", $cached
[inline-code-end]

---