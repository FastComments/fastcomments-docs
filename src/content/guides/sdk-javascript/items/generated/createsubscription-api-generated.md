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
const tenantId: string = 'acme-corp-tenant-4f3e2b1a';
const createData: CreateAPIUserSubscriptionData = {
  userId: 'user_987654',
  planId: 'pro_monthly',
  paymentMethodId: 'pm_1Hh2K4Lx',
  startDate: '2025-12-01T00:00:00Z',
  trialDays: 14,
  metadata: { campaign: 'black_friday_2025' }
} as CreateAPIUserSubscriptionData;
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createData);
[inline-code-end]
