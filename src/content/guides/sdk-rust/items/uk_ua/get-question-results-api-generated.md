## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Ні |  |
| user_id | String | Ні |  |
| start_date | String | Ні |  |
| question_id | String | Ні |  |
| question_ids | String | Ні |  |
| skip | f64 | Ні |  |

## Response

Повертає: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_response.rs)

## Example

[inline-code-attrs-start title = 'Приклад get_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: Some("news/article".to_string()),
        user_id: Some("user-12345".to_string()),
        start_date: Some("2023-01-01".to_string()),
        question_id: Some("q-987".to_string()),
        question_ids: Some("q-1,q-2,q-3".to_string()),
        skip: Some(10.0),
    };
    let _response = get_question_results(configuration, params).await?;
    Ok(())
}
[inline-code-end]