## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | No |  |
| user_id | String | No |  |
| start_date | String | No |  |
| question_id | String | No |  |
| question_ids | String | No |  |
| skip | f64 | No |  |

## Response

Returns: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_question_results Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetQuestionResults200Response, Error> {
    let params: GetQuestionResultsParams = GetQuestionResultsParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: Some(String::from("news/article/2026/01/12/major-update")),
        user_id: Some(String::from("user_12345")),
        start_date: Some(String::from("2026-01-01T00:00:00Z")),
        question_id: Some(String::from("q_abc123")),
        question_ids: Some(String::from("q_abc123,q_def456")),
        skip: Some(10.0),
    };
    let response: GetQuestionResults200Response = get_question_results(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
