## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| question_id | String | No |  |
| question_ids | Vec<String> | No |  |
| url_id | String | No |  |
| time_bucket | models::AggregateTimeBucket | No |  |
| start_date | String | No |  |
| force_recalculate | bool | No |  |

## Response

Returns: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

## Example

[inline-code-attrs-start title = 'aggregate_question_results Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<AggregateQuestionResults200Response, Error> {
    let params: AggregateQuestionResultsParams = AggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("satisfaction_rating".to_string()),
        question_ids: Some(vec!["satisfaction_rating".to_string(), "product_feedback".to_string()]),
        url_id: Some("news/article/2025/product-launch".to_string()),
        time_bucket: Some(models::AggregateTimeBucket::Daily),
        start_date: Some("2025-01-01T00:00:00Z".to_string()),
        force_recalculate: Some(true),
    };
    let response: AggregateQuestionResults200Response = aggregate_question_results(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
