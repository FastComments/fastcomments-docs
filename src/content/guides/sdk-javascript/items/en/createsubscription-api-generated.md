## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Yes |  |

## Response

Returns: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'createSubscription Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f7b3c2a";
const couponCode: string | undefined = undefined;

const subscriptionData: CreateAPIUserSubscriptionData = {
  userId: "user_81a2b4",
  planId: "pro_monthly",
  startDate: "2026-06-01",
  quantity: 1,
  metadata: { source: "website" },
  ...(couponCode ? { couponCode } : {})
};

const response: CreateSubscriptionAPIResponse = await createSubscription(tenantId, subscriptionData);
[inline-code-end]
