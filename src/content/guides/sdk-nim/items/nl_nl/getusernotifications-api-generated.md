---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| pageSize | int | Nee |  |
| afterId | string | Nee |  |
| includeContext | bool | Nee |  |
| afterCreatedAt | int64 | Nee |  |
| unreadOnly | bool | Nee |  |
| dmOnly | bool | Nee |  |
| noDm | bool | Nee |  |
| includeTranslations | bool | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getUserNotifications Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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