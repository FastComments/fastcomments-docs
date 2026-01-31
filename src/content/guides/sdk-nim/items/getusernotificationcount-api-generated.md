## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`Option[GetUserNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count200response.nim)

## Example

[inline-code-attrs-start title = 'getUserNotificationCount Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotificationCount(tenantId = "fastcomments-tenant-001", sso = "")
if response.isSome:
  let result = response.get()
  echo "Notification count response: ", repr(result)
else:
  echo "Request failed with status: ", $httpResponse.status
[inline-code-end]
