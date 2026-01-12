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
async fn run() -> Result<(), Error> {
    let params: AggregateQuestionResultsParams = AggregateQuestionResultsParams {
        tenant_id: String::from("acme-corp-tenant"),
        question_id: Some(String::from("q-12345")),
        question_ids: Some(vec![String::from("q-12345"), String::from("q-67890")]),
        url_id: Some(String::from("news/article/2026/01/12/breaking")),
        time_bucket: Some(models::AggregateTimeBucket::Daily),
        start_date: Some(String::from("2026-01-01T00:00:00Z")),
        force_recalculate: Some(true),
    };
    let aggregation: AggregateQuestionResults200Response =
        aggregate_question_results(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
