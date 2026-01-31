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
const tenantId: string = 'tenant_acme_001';
const parentTenantId: string = 'org_global_acme';
const includeStats: boolean = true;
const aggregationRequest: AggregationRequest = {
  operations: [{ type: 'COUNT' as unknown as AggregationOpType, field: 'articleId' } as AggregationOperation],
  predicates: [{ field: 'status', operator: 'EQ', value: 'published' } as QueryPredicate],
  groupBy: ['articleId'],
  limit: 25,
  sort: [{ field: 'count', direction: 'DESC' } as AggregationRequestSort]
};
const result: AggregationResponse = await aggregate(tenantId, aggregationRequest, parentTenantId, includeStats);
[inline-code-end]
