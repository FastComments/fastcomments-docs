Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Response

Returns: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

## Example

[inline-code-attrs-start title = 'aggregate Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const parentTenantId: string = 'org_98765';
  const includeStats: boolean = true;
  const aggregationRequest: AggregationRequest = {
    predicates: [
      { field: 'createdAt', operator: 'gte', value: '2025-01-01T00:00:00Z' } as QueryPredicate
    ],
    operations: [
      { type: 'COUNT' as AggregationOpType, field: 'status' } as AggregationOperation
    ],
    sort: [{ field: 'count', direction: 'desc' }] as AggregationRequestSort,
    limit: 25
  };
  const response: AggregationResponse = await aggregate(tenantId, aggregationRequest, parentTenantId, includeStats);
  console.log(response);
})();
[inline-code-end]
