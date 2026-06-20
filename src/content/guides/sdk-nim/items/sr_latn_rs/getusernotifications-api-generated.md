## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| pageSize | int | Ne |  |
| afterId | string | Ne |  |
| includeContext | bool | Ne |  |
| afterCreatedAt | int64 | Ne |  |
| unreadOnly | bool | Ne |  |
| dmOnly | bool | Ne |  |
| noDm | bool | Ne |  |
| includeTranslations | bool | Ne |  |
| includeTenantNotifications | bool | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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