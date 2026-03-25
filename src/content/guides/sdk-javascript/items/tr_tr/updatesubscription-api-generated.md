## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Evet |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateSubscriptionAPIResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateSubscription Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2c';
const subscriptionId: string = 'sub_7641a2b3';
const updateData: UpdateAPIUserSubscriptionData = {
  status: 'active',
  planId: 'pro_annual',
  autoRenew: true,
  renewalDate: '2026-04-15T00:00:00Z',
  metadata: { upgradedBy: 'billing-team' }
};
const userId: string = 'user_215';
const result: UpdateSubscriptionAPIResponse = await updateSubscription(tenantId, subscriptionId, updateData, userId);
[inline-code-end]

---