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
let params = BulkAggregateQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    bulk_aggregate_question_results_request: models::BulkAggregateQuestionResultsRequest {
        questions: vec![
            models::BulkAggregateQuestionItem { question_id: "satisfaction".to_string() },
            models::BulkAggregateQuestionItem { question_id: "recommendation".to_string() },
        ],
        start_time: "2025-10-01T00:00:00Z".to_string(),
        end_time: "2025-10-31T23:59:59Z".to_string(),
    },
    force_recalculate: Some(true),
};
let aggregated: BulkAggregateQuestionResults200Response = bulk_aggregate_question_results(&configuration, params).await?;
[inline-code-end]
