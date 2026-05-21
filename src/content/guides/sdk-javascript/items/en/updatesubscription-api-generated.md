## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Yes |  |
| userId | string | No |  |

## Response

Returns: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateSubscriptionAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'updateSubscription Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a3f9d6b2-7c4e-4b1a-9f2d-0e6c8a1b2d3f';
const id: string = 'sub_6f4b2a9e-3c1d-4e8f-9b2a-7d5c3e1f0a8b';
const userId: string = 'user_58b2d4f1-6a7c-4e9b-8d1f-2c3a5b6e7f9d';
const updateData: UpdateAPIUserSubscriptionData = {
  planId: 'business_annual',
  seats: 25,
  autoRenew: false,
  nextBillingDate: '2026-08-01'
};
const result: UpdateSubscriptionAPIResponse = await updateSubscription(tenantId, id, updateData, userId);
[inline-code-end]
