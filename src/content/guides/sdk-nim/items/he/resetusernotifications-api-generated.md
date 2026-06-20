## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| afterId | string | לא |  |
| afterCreatedAt | int64 | לא |  |
| unreadOnly | bool | לא |  |
| dmOnly | bool | לא |  |
| noDm | bool | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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