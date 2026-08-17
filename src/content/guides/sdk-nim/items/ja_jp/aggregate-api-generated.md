Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations. Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

ドキュメントをグループ化（groupBy が提供されている場合）し、複数の操作を適用して集計します。sum、countDistinct、avg などのさまざまな操作がサポートされています。

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| aggregationRequest | AggregationRequest | いいえ |  |
| options | AggregateOptions | いいえ |  |

## Response

Returns: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Example

[inline-code-attrs-start title = '集計例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]