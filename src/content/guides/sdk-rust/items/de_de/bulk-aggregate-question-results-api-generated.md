## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenant_id | String | Ja |  |
| bulk_aggregate_question_results_request | models::BulkAggregateQuestionResultsRequest | Ja |  |
| force_recalculate | bool | Nein |  |

## Antwort

Rückgabe: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_aggregate_question_results_response.rs)

## Beispiel

[inline-code-attrs-start title = 'bulk_aggregate_question_results Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let request = models::BulkAggregateQuestionResultsRequest {
        question_ids: vec!["q123".into(), "q456".into()],
        time_bucket: "daily".into(),
    };
    let params = BulkAggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".into(),
        bulk_aggregate_question_results_request: request,
        force_recalculate: Some(true),
    };
    let _response = bulk_aggregate_question_results(&configuration, params).await?;
    Ok(())
}
[inline-code-end]