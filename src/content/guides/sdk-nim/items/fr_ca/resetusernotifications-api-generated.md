## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| afterId | string | Non |  |
| afterCreatedAt | int64 | Non |  |
| unreadOnly | bool | Non |  |
| dmOnly | bool | Non |  |
| noDm | bool | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = int64(0),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)

if response.isSome:
  let result = response.get()
[inline-code-end]

---