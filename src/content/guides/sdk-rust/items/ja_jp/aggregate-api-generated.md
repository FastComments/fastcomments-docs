ドキュメントをグループ化（groupBy が指定されている場合）して複数の操作を適用することで集計します。sum、countDistinct、avg などのさまざまな操作がサポートされています。

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| aggregation_request | models::AggregationRequest | はい |  |
| parent_tenant_id | String | いいえ |  |
| include_stats | bool | いいえ |  |

## レスポンス

戻り値: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## 例

[inline-code-attrs-start title = 'aggregate の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = AggregateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    aggregation_request: models::AggregationRequest {
        predicates: Some(vec![
            models::QueryPredicate {
                field: "path".to_string(),
                operator: "EQUALS".to_string(),
                values: Some(vec![
                    models::QueryPredicateValue { value: "news/article".to_string() }
                ]),
            }
        ]),
        operations: vec![
            models::AggregationOperation {
                op_type: models::AggregationOpType::COUNT,
                field: Some("comment_id".to_string()),
                alias: Some("total_comments".to_string()),
            }
        ],
        sort: Some(vec![
            models::AggregationRequestSort { field: "total_comments".to_string(), direction: "DESC".to_string() }
        ]),
    },
    parent_tenant_id: Some("acme-parent-tenant".to_string()),
    include_stats: Some(true),
};
let aggregate_response: AggregateResponse = aggregate(&configuration, params).await?;
[inline-code-end]

---