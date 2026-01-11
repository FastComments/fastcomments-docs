## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| question_id | String | No |  |
| question_ids | Vec<String> | No |  |
| url_id | String | No |  |
| start_date | String | No |  |
| force_recalculate | bool | No |  |
| min_value | f64 | No |  |
| max_value | f64 | No |  |
| limit | f64 | No |  |

## Response

Returns: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

## Example

[inline-code-attrs-start title = 'combine_comments_with_question_results Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CombineCommentsWithQuestionResultsParams = CombineCommentsWithQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("q-98765".to_string()),
        question_ids: Some(vec!["q-98765".to_string(), "q-12345".to_string()]),
        url_id: Some("news/technology/2025/rust-release".to_string()),
        start_date: Some("2025-01-01T00:00:00Z".to_string()),
        force_recalculate: Some(true),
        min_value: Some(0.0),
        max_value: Some(1.0),
        limit: Some(100.0),
    };
    let resp: CombineCommentsWithQuestionResults200Response =
        combine_comments_with_question_results(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
