通过对文档进行分组（如果提供了 groupBy）并应用多个操作来聚合文档。
支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| aggregation_request | models::AggregationRequest | 是 |  |
| parent_tenant_id | String | 否 |  |
| include_stats | bool | 否 |  |

## 响应

返回: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## 示例

[inline-code-attrs-start title = 'aggregate 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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