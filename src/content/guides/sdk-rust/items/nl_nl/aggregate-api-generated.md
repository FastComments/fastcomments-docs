Aggregeert documenten door ze te groeperen (als groupBy is provided) en meerdere bewerkingen toe te passen.
Verschillende bewerkingen (bijv. sum, countDistinct, avg, enz.) worden ondersteund.

## Parameters

| Naam | Type | Vereist | Omschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| aggregation_request | models::AggregationRequest | Ja |  |
| parent_tenant_id | String | Nee |  |
| include_stats | bool | Nee |  |

## Antwoord

Retourneert: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'aggregate Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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