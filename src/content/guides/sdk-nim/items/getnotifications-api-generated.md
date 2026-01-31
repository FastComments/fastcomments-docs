## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | Yes |  |
| fromCommentId | string | No |  |
| viewed | bool | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications200response.nim)

## Example

[inline-code-attrs-start title = 'getNotifications Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(
  tenantId = "my-tenant-123",
  userId = "user-456",
  urlId = "news/latest-updates",
  fromCommentId = "cmt-789",
  viewed = false,
  skip = 0.0
)
if response.isSome:
  let notifications = response.get()
  echo "Received notifications: ", notifications
else:
  echo "No notifications. HTTP status: ", $httpResponse.status
[inline-code-end]
