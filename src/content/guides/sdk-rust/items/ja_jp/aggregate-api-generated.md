ドキュメントをグループ化して（groupBy が提供されている場合）複数の操作を適用して集計します。sum、countDistinct、avg など、さまざまな操作をサポートしています。

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| aggregation_request | models::AggregationRequest | はい |  |
| parent_tenant_id | String | いいえ |  |
| include_stats | bool | いいえ |  |

## レスポンス

戻り値: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---