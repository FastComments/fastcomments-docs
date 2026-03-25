## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Да |  |
| userId | string | Не |  |

## Отговор

Връща: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateSubscriptionAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за updateSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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