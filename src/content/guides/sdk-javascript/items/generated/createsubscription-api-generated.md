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
(async () => {
  const tenantId: string = "fc_tenant_7f3b2a";
  const subscriptionData: CreateAPIUserSubscriptionData = {
    userId: "user_84c9",
    planId: "pro_annual",
    startDate: "2026-01-15T12:00:00Z",
    autoRenew: true,
    paymentMethodId: "pm_visa_9876",
    trialEndsAt: "2026-02-15T12:00:00Z", // optional field demonstrated
    metadata: { signupSource: "marketing_campaign" } // optional field demonstrated
  };
  const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, subscriptionData);
  console.log(result);
})();
[inline-code-end]
