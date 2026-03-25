## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenant_id | String | Ja |  |
| question_id | String | Nee |  |
| question_ids | Vec<String> | Nee |  |
| url_id | String | Nee |  |
| time_bucket | models::AggregateTimeBucket | Nee |  |
| start_date | String | Nee |  |
| force_recalculate | bool | Nee |  |

## Respons

Retourneert: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'aggregate_question_results Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: AggregateQuestionResultsParams = AggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("satisfaction-8".to_string()),
        question_ids: Some(vec!["satisfaction-8".to_string(), "recommendation-3".to_string()]),
        url_id: Some("news/article/2026/ai-announce".to_string()),
        time_bucket: Some(models::AggregateTimeBucket::Daily),
        start_date: Some("2026-03-01T00:00:00Z".to_string()),
        force_recalculate: Some(true),
    };
    let aggregation: AggregateQuestionResults200Response = aggregate_question_results(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---