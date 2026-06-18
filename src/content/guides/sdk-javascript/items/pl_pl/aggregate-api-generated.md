Agreguje dokumenty, grupując je (jeśli podano groupBy) i stosując wiele operacji. Obsługiwane są różne operacje (np. sum, countDistinct, avg itp.).

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| aggregationRequest | AggregationRequest | Tak |  |
| parentTenantId | string | Nie |  |
| includeStats | boolean | Nie |  |

## Odpowiedź

Zwraca: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład agregacji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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