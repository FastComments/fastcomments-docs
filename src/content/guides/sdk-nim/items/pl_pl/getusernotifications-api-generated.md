## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| pageSize | int | Nie |  |
| afterId | string | Nie |  |
| includeContext | bool | Nie |  |
| afterCreatedAt | int64 | Nie |  |
| unreadOnly | bool | Nie |  |
| dmOnly | bool | Nie |  |
| noDm | bool | Nie |  |
| includeTranslations | bool | Nie |  |
| includeTenantNotifications | bool | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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