## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateNotificationBody | UpdateNotificationBody | No |  |
| userId | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateNotification Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateNotification(
  tenantId = "news-tenant-42",
  id = "notif-98765",
  updateNotificationBody = default(UpdateNotificationBody),
  userId = "editor-123"
)

if response.isSome:
  let result = response.get()
  echo "Notification updated:", result
else:
  echo "No response body, HTTP status:", httpResponse.status.code
[inline-code-end]
