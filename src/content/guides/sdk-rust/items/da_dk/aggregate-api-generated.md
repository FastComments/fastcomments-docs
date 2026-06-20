---
Aggregerer dokumenter ved at gruppere dem (hvis groupBy er angivet) og anvende flere operationer.
Forskellige operationer (f.eks. sum, countDistinct, avg osv.) understøttes.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| aggregation_request | models::AggregationRequest | Ja |  |
| parent_tenant_id | String | Nej |  |
| include_stats | bool | Nej |  |

## Svar

Returnerer: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Eksempel

[inline-code-attrs-start title = 'aggregate-eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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