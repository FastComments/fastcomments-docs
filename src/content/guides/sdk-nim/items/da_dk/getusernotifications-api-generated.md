## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| pageSize | int | Nej |  |
| afterId | string | Nej |  |
| includeContext | bool | Nej |  |
| afterCreatedAt | int64 | Nej |  |
| unreadOnly | bool | Nej |  |
| dmOnly | bool | Nej |  |
| noDm | bool | Nej |  |
| includeTranslations | bool | Nej |  |
| includeTenantNotifications | bool | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getUserNotifications Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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