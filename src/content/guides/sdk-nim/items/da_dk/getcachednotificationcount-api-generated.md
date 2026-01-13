## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |

## Svar

Returnerer: [`Option[GetCachedNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count200response.nim)

## Eksempel

[inline-code-attrs-start title = 'getCachedNotificationCount Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "notif-thread-2026")
if response.isSome:
  let cached = response.get()
  echo "Cached notification count: ", $cached
[inline-code-end]