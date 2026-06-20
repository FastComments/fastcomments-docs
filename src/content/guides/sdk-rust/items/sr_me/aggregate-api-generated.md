Агрегира документе груписањем (ако је groupBy наведен) и примјењујући више операција.
Подржане су различите операције (нпр. sum, countDistinct, avg, итд.).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| aggregation_request | models::AggregationRequest | Да |  |
| parent_tenant_id | String | Не |  |
| include_stats | bool | Не |  |

## Одговор

Враћа: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример агрегирања'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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