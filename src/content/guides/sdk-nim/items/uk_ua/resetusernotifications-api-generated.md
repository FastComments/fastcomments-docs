## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| afterId | string | Ні |  |
| afterCreatedAt | int64 | Ні |  |
| unreadOnly | bool | Ні |  |
| dmOnly | bool | Ні |  |
| noDm | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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