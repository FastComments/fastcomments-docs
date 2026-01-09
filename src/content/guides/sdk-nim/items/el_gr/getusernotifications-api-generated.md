## Παράμετροι

| Name | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| pageSize | int | Όχι |  |
| afterId | string | Όχι |  |
| includeContext | bool | Όχι |  |
| afterCreatedAt | int64 | Όχι |  |
| unreadOnly | bool | Όχι |  |
| dmOnly | bool | Όχι |  |
| noDm | bool | Όχι |  |
| includeTranslations | bool | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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