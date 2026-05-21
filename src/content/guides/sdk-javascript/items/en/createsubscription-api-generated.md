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
const tenantId: string = "acme-enterprises-987";
const createData: CreateAPIUserSubscriptionData = {
  apiUserId: "user_72f4a1",
  planId: "enterprise_annual",
  startDate: "2026-06-01T00:00:00Z",
  autoRenew: true,
  trialPeriodDays: 30
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createData);
[inline-code-end]
