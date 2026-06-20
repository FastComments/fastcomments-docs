## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| pageSize | int | Nein |  |
| afterId | string | Nein |  |
| includeContext | bool | Nein |  |
| afterCreatedAt | int64 | Nein |  |
| unreadOnly | bool | Nein |  |
| dmOnly | bool | Nein |  |
| noDm | bool | Nein |  |
| includeTranslations | bool | Nein |  |
| includeTenantNotifications | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getUserNotifications Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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