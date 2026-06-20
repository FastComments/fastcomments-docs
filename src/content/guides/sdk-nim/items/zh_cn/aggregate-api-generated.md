通过对文档进行分组（如果提供了 groupBy）并应用多个操作来聚合文档。
支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| aggregationRequest | AggregationRequest | 否 |  |
| parentTenantId | string | 否 |  |
| includeStats | bool | 否 |  |

## 响应

返回：[`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## 示例

[inline-code-attrs-start title = 'aggregate 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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