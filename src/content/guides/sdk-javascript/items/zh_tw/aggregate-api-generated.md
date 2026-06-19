---
透過群組文件（如果提供了 groupBy）並套用多個操作來彙總文件。
支援不同的操作（例如 sum、countDistinct、avg 等）。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| aggregationRequest | AggregationRequest | 是 |  |
| parentTenantId | string | 否 |  |
| includeStats | boolean | 否 |  |

## 回應

回傳: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## 範例

[inline-code-attrs-start title = 'aggregate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_72b3";
const parentTenantId: string = "parent_acme_corp";
const aggregationRequest: AggregationRequest = {
  groupBy: ["postId"],
  predicates: [
    { field: "status", operator: "EQ", value: { stringValue: "published" } as QueryPredicateValue }
  ],
  operations: [
    { type: AggregationOpType.COUNT, field: "id", alias: "commentCount" } as AggregationOperation
  ],
  sort: [{ field: "commentCount", direction: "DESC" } as AggregationRequestSort],
  limit: 25
};
const response: AggregateResponse = await aggregate(tenantId, aggregationRequest, parentTenantId, true);
[inline-code-end]

---