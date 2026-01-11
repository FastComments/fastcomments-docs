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
let createData = CreateAPIUserSubscriptionData(
  email = "john.doe@example.com",
  urlId = "news/politics/2025-election",
  subscribed = true,
  frequency = "immediate",
  tags = @["politics", "election"]
)

let (response, httpResponse) = client.createSubscription(tenantId = "my-tenant-123", createAPIUserSubscriptionData = createData)

if response.isSome:
  let subscription = response.get()
  echo "Created subscription for: ", subscription.email
else:
  echo "Subscription creation failed, status: ", $httpResponse.statusCode
[inline-code-end]
