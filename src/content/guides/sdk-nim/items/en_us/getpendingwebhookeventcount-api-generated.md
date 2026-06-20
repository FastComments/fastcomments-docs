## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| externalId | string | No |  |
| eventType | string | No |  |
| domain | string | No |  |
| attemptCountGT | float64 | No |  |

## Response

Returns: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEventCount Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEventCount(
  tenantId = "my-tenant-123",
  commentId = "cmt-456abc",
  externalId = "ext-7890",
  eventType = "comment_created",
  domain = "news.example.com",
  attemptCountGT = 2.0
)

if response.isSome:
  let pending = response.get()
  echo pending
else:
  echo "No pending webhook event count returned; HTTP status: ", httpResponse.status
[inline-code-end]
