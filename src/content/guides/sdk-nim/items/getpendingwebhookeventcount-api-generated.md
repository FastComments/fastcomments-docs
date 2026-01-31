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

Returns: [`Option[GetPendingWebhookEventCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count200response.nim)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEventCount Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEventCount(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  externalId = "",
  eventType = "",
  domain = "",
  attemptCountGT = 0.0
)
if response.isSome:
  let pending = response.get()
  echo "Pending webhook event count received: ", $pending
else:
  echo "No response body; HTTP status: ", $httpResponse.status
[inline-code-end]
