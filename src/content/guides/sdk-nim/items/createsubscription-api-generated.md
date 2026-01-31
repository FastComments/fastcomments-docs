## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | No |  |

## Response

Returns: [`Option[CreateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_subscription_api_response.nim)

## Example

[inline-code-attrs-start title = 'createSubscription Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let subscriptionData = CreateAPIUserSubscriptionData(
  email = "jane.doe@example.com",
  urlId = "news/local-election-2026",
  frequency = "daily",
  categories = @["politics", "local"],
  active = true
)

let (response, httpResponse) = client.createSubscription(tenantId = "my-tenant-123", createAPIUserSubscriptionData = subscriptionData)

if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]
