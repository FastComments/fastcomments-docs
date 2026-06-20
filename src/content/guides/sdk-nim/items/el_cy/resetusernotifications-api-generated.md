## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| afterCreatedAt | int64 | Όχι |  |
| unreadOnly | bool | Όχι |  |
| dmOnly | bool | Όχι |  |
| noDm | bool | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = 0'i64,
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)
if response.isSome:
  let resetResp = response.get()
  echo "ResetUserNotificationsResponse received"
else:
  echo "No ResetUserNotificationsResponse"
[inline-code-end]

---