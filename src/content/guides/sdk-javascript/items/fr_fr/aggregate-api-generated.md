---
Regroupe les documents (si groupBy est fourni) et applique plusieurs opérations.
Différentes opérations (par ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Réponse

Renvoie: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple pour aggregate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_78a9';
const parentTenantId: string = 'parent_tenant_01';
const includeStats: boolean = true;
const aggregationRequest: AggregationRequest = {
  operation: { type: 'COUNT' },
  groupBy: ['pageUrl'],
  predicate: { field: 'status', operator: 'EQUALS', value: 'approved' },
  sort: [{ field: 'count', direction: 'DESC' }],
  limit: 25
};
const result: Aggregate200Response = await aggregate(tenantId, aggregationRequest, parentTenantId, includeStats);
[inline-code-end]

---