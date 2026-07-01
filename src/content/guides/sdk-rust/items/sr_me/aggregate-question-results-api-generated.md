## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| question_id | String | No |  |
| question_ids | Vec<String> | No |  |
| url_id | String | No |  |
| time_bucket | models::AggregateTimeBucket | No |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | No |  |
| force_recalculate | bool | No |  |

## Odgovor

Vraća: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_response.rs)

## Primer

[inline-code-attrs-start title = 'aggregate_question_results Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = AggregateQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    question_id: Some("question-123".to_string()),
    question_ids: Some(vec!["question-123".to_string(), "question-456".to_string()]),
    url_id: Some("news/article".to_string()),
    time_bucket: Some(models::AggregateTimeBucket::Day),
    start_date: Some(chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00+00:00").unwrap()),
    force_recalculate: Some(true),
};

let response = aggregate_question_results(&configuration, params).await?;
[inline-code-end]