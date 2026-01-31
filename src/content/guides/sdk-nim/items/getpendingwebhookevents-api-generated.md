## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| externalId | string | No |  |
| eventType | string | No |  |
| domain | string | No |  |
| attemptCountGT | float64 | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetPendingWebhookEvents_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events200response.nim)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEvents Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  externalId = "",
  eventType = "",
  domain = "",
  attemptCountGT = 0.0,
  skip = 0.0
)

if response.isSome:
  let pending = response.get()
  echo "Pending webhook events fetched successfully"
[inline-code-end]
