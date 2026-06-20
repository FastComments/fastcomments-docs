## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| bulk_aggregate_question_results_request | models::BulkAggregateQuestionResultsRequest | Да |  |
| force_recalculate | bool | Не |  |

## Одговор

Враћа: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_aggregate_question_results_response.rs)

## Пример

[inline-code-attrs-start title = 'bulk_aggregate_question_results Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: BulkAggregateQuestionResultsParams = BulkAggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        bulk_aggregate_question_results_request: models::BulkAggregateQuestionResultsRequest {
            items: vec![
                models::BulkAggregateQuestionItem {
                    question_id: "article-engagement".to_string(),
                    path: "news/article/987".to_string(),
                    include_subpaths: true,
                }
            ],
            time_bucket: models::AggregateTimeBucket::Daily,
            range_start: "2026-05-01T00:00:00Z".to_string(),
            range_end: "2026-05-31T23:59:59Z".to_string(),
        },
        force_recalculate: Some(true),
    };
    let result: BulkAggregateQuestionResultsResponse = bulk_aggregate_question_results(&configuration, params).await?;
    println!("{:#?}", result);
    Ok(())
}
[inline-code-end]

---