Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| aggregation_request | models::AggregationRequest | Yes |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Response

Returns: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

## Example

[inline-code-attrs-start title = 'aggregate Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: AggregateParams = AggregateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    aggregation_request: models::AggregationRequest {
        filters: Some(vec![
            models::QueryPredicate {
                field: "path".to_string(),
                operator: "equals".to_string(),
                value: models::QueryPredicateValue::String("news/article".to_string()),
            },
        ]),
        operations: vec![
            models::AggregationOperation {
                op: models::AggregationOpType::Count,
                field: None,
            },
        ],
        group_by: Some(vec!["author_id".to_string()]),
        sort: Some(vec![
            models::AggregationRequestSort { field: "count".to_string(), direction: "desc".to_string() },
        ]),
        limit: Some(100),
    },
    parent_tenant_id: Some("acme-global".to_string()),
    include_stats: Some(true),
};
let aggregation: AggregationResponse = aggregate(&configuration, params).await?;
[inline-code-end]
