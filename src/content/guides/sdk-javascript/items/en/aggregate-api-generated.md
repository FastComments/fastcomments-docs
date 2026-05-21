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
const tenantId: string = "acme_corp_123";
const parentTenantId: string = "global_acme_1";
const includeStats: boolean = true;

const predicateValue: QueryPredicateValue = { stringValue: "active" };
const predicate: QueryPredicate = { field: "status", operator: "EQ", value: predicateValue };
const operation: AggregationOperation = { type: "COUNT", field: "id", alias: "total" };
const sort: AggregationRequestSort = { field: "total", direction: "DESC" };

const aggregationRequest: AggregationRequest = {
  query: { predicates: [predicate] },
  operations: [operation],
  sort: [sort],
  limit: 100
};

const response: AggregationResponse = await aggregate(tenantId, aggregationRequest, parentTenantId, includeStats);
[inline-code-end]
