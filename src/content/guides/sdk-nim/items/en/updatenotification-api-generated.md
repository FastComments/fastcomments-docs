## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateNotificationBody | UpdateNotificationBody | No |  |
| userId | string | No |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'updateNotification Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateNotification(
  tenantId = "my-tenant-123",
  id = "notif-456",
  updateNotificationBody = UpdateNotificationBody(
    enabled = true,
    channels = @["email", "push"],
    frequency = "immediate"
  ),
  userId = "user-789"
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]
