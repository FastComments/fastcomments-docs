## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| options | ResetUserNotificationsOptions | Ні |  |

## Response

Повертає: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Example

[inline-code-attrs-start title = 'resetUserNotifications Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  options = ResetUserNotificationsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]