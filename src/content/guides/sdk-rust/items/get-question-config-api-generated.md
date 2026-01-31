## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_config_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_question_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let cfg: configuration::Configuration = configuration::Configuration {
        base_path: Some("https://api.fastcomments.com".to_string()),
        ..Default::default()
    };
    let params: GetQuestionConfigParams = GetQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2026/01/13-article-12345".to_string(),
    };
    let response: GetQuestionConfig200Response = get_question_config(&cfg, params).await?;
    Ok(())
}
[inline-code-end]
