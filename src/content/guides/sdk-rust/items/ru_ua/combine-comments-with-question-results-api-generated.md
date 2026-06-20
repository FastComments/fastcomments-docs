## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| question_id | String | Нет |  |
| question_ids | Vec<String> | Нет |  |
| url_id | String | Нет |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | Нет |  |
| force_recalculate | bool | Нет |  |
| min_value | f64 | Нет |  |
| max_value | f64 | Нет |  |
| limit | f64 | Нет |  |

## Ответ

Возвращает: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_question_results_with_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример combine_comments_with_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CombineCommentsWithQuestionResultsParams = CombineCommentsWithQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    question_id: None,
    question_ids: Some(vec!["product-satisfaction".to_string(), "support-response".to_string()]),
    url_id: Some("news/article-42".to_string()),
    start_date: Some(chrono::FixedOffset::east(0).ymd(2025, 12, 01).and_hms(08, 00, 00)),
    force_recalculate: Some(true),
    min_value: Some(0.0),
    max_value: Some(1.0),
    limit: Some(250.0),
};
let response: CombineQuestionResultsWithCommentsResponse = combine_comments_with_question_results(&configuration, params).await?;
[inline-code-end]

---