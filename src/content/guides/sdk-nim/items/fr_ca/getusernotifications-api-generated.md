## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| pageSize | int | Non |  |
| afterId | string | Non |  |
| includeContext | bool | Non |  |
| afterCreatedAt | int64 | Non |  |
| unreadOnly | bool | Non |  |
| dmOnly | bool | Non |  |
| noDm | bool | Non |  |
| includeTranslations | bool | Non |  |
| includeTenantNotifications | bool | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  pageSize = 0,
  afterId = "",
  includeContext = false,
  afterCreatedAt = 0,
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  includeTenantNotifications = false,
  sso = ""
)

if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---