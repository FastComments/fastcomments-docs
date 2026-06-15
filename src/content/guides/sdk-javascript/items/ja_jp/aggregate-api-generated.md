ドキュメントをグループ化して（groupBy が指定されている場合）複数の操作を適用して集計します。  
さまざまな操作（例: sum、countDistinct、avg など）をサポートします。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| aggregationRequest | AggregationRequest | はい |  |
| parentTenantId | string | いいえ |  |
| includeStats | boolean | いいえ |  |

## Response

戻り値: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Example

[inline-code-attrs-start title = 'aggregate の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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