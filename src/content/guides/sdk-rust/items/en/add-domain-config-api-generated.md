## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| add_domain_config_params | models::AddDomainConfigParams | Yes |  |

## Response

Returns: [`AddDomainConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_domain_config_200_response.rs)

## Example

[inline-code-attrs-start title = 'add_domain_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<AddDomainConfig200Response, Error> {
    let params: AddDomainConfigParams = AddDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        add_domain_config_params: models::AddDomainConfigParams {
            domain: "comments.acme.com".to_string(),
            path: Some("/news/article".to_string()),
            enable_cors: Some(true),
            allowed_origins: Some(vec![
                "https://acme.com".to_string(),
                "https://www.acme.com".to_string()
            ]),
            sso_required: Some(false),
        },
    };
    let response: AddDomainConfig200Response = add_domain_config(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
