## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| bulk_aggregate_question_results_request | models::BulkAggregateQuestionResultsRequest | Yes |  |
| force_recalculate | bool | No |  |

## Response

Returns: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_aggregate_question_results_200_response.rs)

## Example

[inline-code-attrs-start title = 'bulk_aggregate_question_results Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_bulk_aggregate(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: BulkAggregateQuestionResultsParams = BulkAggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        bulk_aggregate_question_results_request: models::BulkAggregateQuestionResultsRequest {
            question_ids: vec!["article_quality".to_string(), "recommendation".to_string()],
            start_time: "2025-01-01T00:00:00Z".to_string(),
            end_time: "2025-01-31T23:59:59Z".to_string(),
            time_bucket: models::AggregateTimeBucket::Daily,
        },
        force_recalculate: Some(true),
    };
    let response: BulkAggregateQuestionResults200Response = bulk_aggregate_question_results(configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]
