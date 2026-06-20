透過分組文件（若提供 groupBy）並套用多個運算以進行彙總。
支援不同的運算（例如 sum、countDistinct、avg 等）。

## 參數

| 名稱 | 類型 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| parentTenantId | string | query | 否 |  |
| includeStats | boolean | query | 否 |  |

## 回應

回傳: [`AggregateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateResponse.swift)

## 範例

[inline-code-attrs-start title = 'aggregate 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let aggregationRequest = AggregationRequest(query: [QueryPredicate(key: "key_example", value: QueryPredicate_value(), _operator: "_operator_example")], resourceName: "resourceName_example", groupBy: ["groupBy_example"], operations: [AggregationOperation(field: "field_example", op: AggregationOpType(), alias: "alias_example", expandArray: false)], sort: AggregationRequest_sort(dir: "dir_example", field: "field_example")) // AggregationRequest | 
let parentTenantId = "parentTenantId_example" // String |  (可選)
let includeStats = true // Bool |  (可選)

DefaultAPI.aggregate(tenantId: tenantId, aggregationRequest: aggregationRequest, parentTenantId: parentTenantId, includeStats: includeStats) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---