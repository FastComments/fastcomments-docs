## פרמטרים

| שם | סוג | דרוש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | ResetUserNotificationsOptions | No |  |

## תגובה

Returns: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  options = ResetUserNotificationsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]