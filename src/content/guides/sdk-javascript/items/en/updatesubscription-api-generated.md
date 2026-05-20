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
(async () => {
  const tenantId: string = 'acme-corp-117';
  const id: string = 'sub_7f3b21';
  const userId: string = 'user_jdoe';
  const updateAPIUserSubscriptionData: UpdateAPIUserSubscriptionData = {
    planId: 'pro-monthly',
    status: 'active',
    startDate: '2026-05-01',
    seats: 10,
    autoRenew: true,
    billingCycle: 'monthly',
    metadata: { source: 'self-serve' }
  };
  const result: UpdateSubscriptionAPIResponse = await updateSubscription(tenantId, id, updateAPIUserSubscriptionData, userId);
  console.log(result);
})();
[inline-code-end]
