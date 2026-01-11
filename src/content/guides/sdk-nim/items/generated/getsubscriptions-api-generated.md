## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`Option[GetSubscriptionsAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_subscriptions_api_response.nim)

## Example

[inline-code-attrs-start title = 'getSubscriptions Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSubscriptions(tenantId = "my-tenant-123", userId = "user-456")
if response.isSome:
  let subscriptions = response.get()
  echo "Received subscriptions for tenant my-tenant-123:"
  echo subscriptions
else:
  echo "No subscriptions found, HTTP status: ", $httpResponse.status
[inline-code-end]
