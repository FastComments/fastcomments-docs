## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Example

[inline-code-attrs-start title = 'resetUserNotificationCount Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if response.isSome:
  let resetInfo = response.get()
  echo "Notifications reset:", resetInfo
else:
  echo "Reset failed"
[inline-code-end]
