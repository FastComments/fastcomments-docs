---
groupBy が提供されている場合にグループ化し、複数の操作を適用してドキュメントを集約します。
sum、countDistinct、avg など、さまざまな操作がサポートされています。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| aggregationRequest | AggregationRequest | はい |  |
| parentTenantId | string | いいえ |  |
| includeStats | boolean | いいえ |  |

## レスポンス

戻り値: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---