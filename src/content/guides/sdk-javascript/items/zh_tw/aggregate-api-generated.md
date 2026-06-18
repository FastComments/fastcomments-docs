透過分組（若提供 groupBy）並應用多個操作來彙總文件。支援不同的操作（例如 sum、countDistinct、avg 等）。

## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| aggregationRequest | AggregationRequest | 是 |  |
| parentTenantId | string | 否 |  |
| includeStats | boolean | 否 |  |

## 回應

回傳：[`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## 範例

[inline-code-attrs-start title = 'aggregate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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