## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | Yes |  |
| fromCommentId | string | No |  |
| viewed | bool | No |  |

## Response

Returns: [`Option[GetNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count200response.nim)

## Example

[inline-code-attrs-start title = 'getNotificationCount Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotificationCount(tenantId = "my-tenant-123", userId = "", urlId = "news/2026/market-analysis", fromCommentId = "", viewed = false)
if response.isSome:
  let countResp = response.get()
  echo "Received notification count response"
else:
  echo "No notification count received, HTTP status: ", $httpResponse.statusCode
[inline-code-end]
