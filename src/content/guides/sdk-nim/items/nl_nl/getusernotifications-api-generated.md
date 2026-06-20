## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| pageSize | int | Nee |  |
| afterId | string | Nee |  |
| includeContext | bool | Nee |  |
| afterCreatedAt | int64 | Nee |  |
| unreadOnly | bool | Nee |  |
| dmOnly | bool | Nee |  |
| noDm | bool | Nee |  |
| includeTranslations | bool | Nee |  |
| includeTenantNotifications | bool | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getUserNotifications Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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