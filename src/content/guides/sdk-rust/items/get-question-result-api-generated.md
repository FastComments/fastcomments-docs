## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_question_result Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_result() -> Result<GetQuestionResult200Response, Error> {
    let tenant: Option<String> = Some("acme-corp-tenant".to_string());
    let id: String = "news/article-2026-01-13-12345".to_string();
    let params = GetQuestionResultParams {
        tenant_id: tenant.unwrap(),
        id,
    };
    let response: GetQuestionResult200Response = get_question_result(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
