## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| pageSize | int | Non |  |
| afterId | string | Non |  |
| includeContext | bool | Non |  |
| afterCreatedAt | int64 | Non |  |
| unreadOnly | bool | Non |  |
| dmOnly | bool | Non |  |
| noDm | bool | Non |  |
| includeTranslations | bool | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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