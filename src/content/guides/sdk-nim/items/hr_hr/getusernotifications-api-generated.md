---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| pageSize | int | Ne |  |
| afterId | string | Ne |  |
| includeContext | bool | Ne |  |
| afterCreatedAt | int64 | Ne |  |
| unreadOnly | bool | Ne |  |
| dmOnly | bool | Ne |  |
| noDm | bool | Ne |  |
| includeTranslations | bool | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Primjer

[inline-code-attrs-start title = 'getUserNotifications Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  pageSize = 50,
  afterId = "notif_9a1b2c3d",
  includeContext = true,
  afterCreatedAt = int64(1699999999000),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  sso = ""
)
if response.isSome:
  let notifications = response.get()
  discard notifications
else:
  discard httpResponse
[inline-code-end]

---