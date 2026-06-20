## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_config_response.rs)

## Example

[inline-code-attrs-start title = 'get_question_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_question_config() -> Result<GetQuestionConfigResponse, Error> {
    let configuration: configuration::Configuration = configuration::Configuration::default();
    let optional_tenant: Option<String> = Some("acme-corp-tenant".to_string());
    let tenant_id: String = optional_tenant.unwrap_or_else(|| "acme-default".to_string());
    let params = GetQuestionConfigParams {
        tenant_id,
        id: "news/article/2026-06-18".to_string(),
    };
    let response: GetQuestionConfigResponse = get_question_config(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
