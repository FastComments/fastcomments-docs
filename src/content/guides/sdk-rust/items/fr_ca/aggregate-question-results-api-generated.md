## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| question_id | String | Non |  |
| question_ids | Vec<String> | Non |  |
| url_id | String | Non |  |
| time_bucket | models::AggregateTimeBucket | Non |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | Non |  |
| force_recalculate | bool | Non |  |

## Réponse

Renvoie : [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_response.rs)

## Exemple

[inline-code-attrs-start title = 'aggregate_question_results Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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