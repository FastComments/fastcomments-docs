## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |

## Risposta

Restituisce: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getSubscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_01';
const userId: string = 'user_76a3b9f2';
const subscriptionsForUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
const subscriptionsForTenant: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
[inline-code-end]

---