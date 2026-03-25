## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |

## Odgovor

Vrne: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getSubscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_01';
const userId: string = 'user_76a3b9f2';
const subscriptionsForUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
const subscriptionsForTenant: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
[inline-code-end]

---