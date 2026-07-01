Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations. Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | No |  |
| options | AggregateOptions | No |  |

## 响应

返回：[`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## 示例

[inline-code-attrs-start title = '聚合示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]