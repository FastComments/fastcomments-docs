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
let aggregation_request: models::AggregationRequest = models::AggregationRequest {
    operations: vec![
        models::AggregationOperation {
            op_type: models::AggregationOpType::COUNT,
            field: Some("comment_id".to_string()),
            alias: Some("total_comments".to_string()),
        }
    ],
    group_by: Some(vec!["article_id".to_string(), "section".to_string()]),
    predicate: Some(models::QueryPredicate {
        field: "status".to_string(),
        operator: "equals".to_string(),
        value: models::QueryPredicateValue::String("published".to_string()),
    }),
    limit: Some(100),
    sort: Some(vec![
        models::AggregationRequestSort { field: "total_comments".to_string(), direction: "desc".to_string() }
    ]),
    ..Default::default()
};

let params: AggregateParams = AggregateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    aggregation_request,
    parent_tenant_id: Some("acme-parent".to_string()),
    include_stats: Some(true),
};

let aggregation_response: models::AggregationResponse = aggregate(&configuration, params).await?;
[inline-code-end]
