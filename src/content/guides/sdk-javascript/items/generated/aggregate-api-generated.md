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
const tenantId: string = "tenant_42";
const parentTenantId: string = "org_parent_7";
const includeStats: boolean = true;
const aggregationRequest: AggregationRequest = {
  operation: { type: "COUNT", field: "commentId" },
  groupBy: ["postId"],
  filters: { predicates: [{ field: "status", operator: "EQ", value: { stringValue: "approved" } }] },
  sort: [{ field: "count", direction: "DESC" }],
  limit: 50
};
const result: AggregationResponse = await aggregate(tenantId, aggregationRequest, parentTenantId, includeStats);
[inline-code-end]
