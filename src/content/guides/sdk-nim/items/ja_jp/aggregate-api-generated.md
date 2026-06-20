---
ドキュメントを集約します。groupBy が指定されている場合はグループ化し、複数の操作を適用します。sum、countDistinct、avg などの操作が利用可能です。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| aggregationRequest | AggregationRequest | いいえ |  |
| parentTenantId | string | いいえ |  |
| includeStats | bool | いいえ |  |

## レスポンス

返却: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## 例

[inline-code-attrs-start title = 'aggregate の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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