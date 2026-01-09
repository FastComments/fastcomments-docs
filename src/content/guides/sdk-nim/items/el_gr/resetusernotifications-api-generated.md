## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| afterCreatedAt | int64 | Όχι |  |
| unreadOnly | bool | Όχι |  |
| dmOnly | bool | Όχι |  |
| noDm | bool | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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