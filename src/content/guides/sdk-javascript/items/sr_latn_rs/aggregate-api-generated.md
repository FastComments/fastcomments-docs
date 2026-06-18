Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Da |  |
| parentTenantId | string | Ne |  |
| includeStats | boolean | Ne |  |

## Odgovor

Vraća: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer aggregate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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