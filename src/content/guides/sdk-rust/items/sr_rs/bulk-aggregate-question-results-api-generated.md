## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| bulk_aggregate_question_results_request | models::BulkAggregateQuestionResultsRequest | Yes |  |
| force_recalculate | bool | No |  |

## Odgovor

Vraća: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_aggregate_question_results_response.rs)

## Primer

[inline-code-attrs-start title = 'bulk_aggregate_question_results Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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