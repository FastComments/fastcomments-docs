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
let (response, httpResponse) = client.createSubscription(tenantId = "my-tenant-123", createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(userId = "user-789", email = "jane.doe@example.com", subscriptionType = "thread", topics = @["news", "world"], active = true))
if response.isSome:
  let created = response.get()
  echo "Subscription created for user: ", created.userId
[inline-code-end]
