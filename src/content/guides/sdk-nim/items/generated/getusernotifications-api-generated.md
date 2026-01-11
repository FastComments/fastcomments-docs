## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| pageSize | int | No |  |
| afterId | string | No |  |
| includeContext | bool | No |  |
| afterCreatedAt | int64 | No |  |
| unreadOnly | bool | No |  |
| dmOnly | bool | No |  |
| noDm | bool | No |  |
| includeTranslations | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Example

[inline-code-attrs-start title = 'getUserNotifications Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  pageSize = 25,
  afterId = "notif-789",
  includeContext = true,
  afterCreatedAt = int64(1700000000),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = true,
  sso = "sso-user-452"
)

if response.isSome:
  let notifications = response.get()
  echo "Received user notifications"
else:
  echo "No notifications available"
[inline-code-end]
