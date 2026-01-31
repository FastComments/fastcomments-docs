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
const tenantId: string = 'tenant_7c9a4b2d';
const subscriptionData: CreateAPIUserSubscriptionData = {
  userId: 'user_4a1c2b',
  planId: 'pro_monthly',
  startDate: '2026-01-15T00:00:00Z',
  autoRenew: true,
  paymentMethod: { type: 'card', brand: 'Visa', last4: '4242' },
  couponCode: 'WELCOME10' // optional parameter
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, subscriptionData);
const createdSubscription: APIUserSubscription | undefined = result.subscription;
[inline-code-end]
