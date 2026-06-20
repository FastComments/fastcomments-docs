透過分組（若提供 groupBy）並套用多個運算來聚合文件。支援不同的運算（例如 sum、countDistinct、avg 等）。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| aggregationRequest | AggregationRequest | 否 |  |
| parentTenantId | string | 否 |  |
| includeStats | bool | 否 |  |

## 回應

回傳：[`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## 範例

[inline-code-attrs-start title = 'aggregate 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(groupBy = @["articleId"], metrics = @["commentCount"], filters = @[], limit = 0),
  parentTenantId = "",
  includeStats = false
)

if response.isSome:
  let agg = response.get()
  discard agg
[inline-code-end]

---