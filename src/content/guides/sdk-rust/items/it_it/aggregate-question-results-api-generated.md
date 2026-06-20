## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| question_id | String | No |  |
| question_ids | Vec<String> | No |  |
| url_id | String | No |  |
| time_bucket | models::AggregateTimeBucket | No |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | No |  |
| force_recalculate | bool | No |  |

## Risposta

Restituisce: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregate_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<AggregateQuestionResultsResponse, Error> {
    let params: AggregateQuestionResultsParams = AggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("q-12345".to_string()),
        question_ids: Some(vec!["q-12345".to_string(), "q-67890".to_string()]),
        url_id: Some("news/article/2026/06/breaking".to_string()),
        time_bucket: Some(models::AggregateTimeBucket::Daily),
        start_date: Some(chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00+00:00").unwrap()),
        force_recalculate: Some(true),
    };
    let response: AggregateQuestionResultsResponse = aggregate_question_results(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---