---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| pageSize | int | Όχι |  |
| afterId | string | Όχι |  |
| includeContext | bool | Όχι |  |
| afterCreatedAt | int64 | Όχι |  |
| unreadOnly | bool | Όχι |  |
| dmOnly | bool | Όχι |  |
| noDm | bool | Όχι |  |
| includeTranslations | bool | Όχι |  |
| includeTenantNotifications | bool | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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