## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| pageSize | int | No |  |
| afterId | string | No |  |
| includeContext | bool | No |  |
| afterCreatedAt | int64 | No |  |
| unreadOnly | bool | No |  |
| dmOnly | bool | No |  |
| noDm | bool | No |  |
| includeTranslations | bool | No |  |
| includeTenantNotifications | bool | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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