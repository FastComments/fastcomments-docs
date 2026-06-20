## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| question_id | String | Ne |  |
| question_ids | Vec<String> | Ne |  |
| url_id | String | Ne |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | Ne |  |
| force_recalculate | bool | Ne |  |
| min_value | f64 | Ne |  |
| max_value | f64 | Ne |  |
| limit | f64 | Ne |  |

## Odgovor

Vraća: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_question_results_with_comments_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer combine_comments_with_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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