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
async fn run() -> Result<(), Error> {
    let params: AddDomainConfigParams = AddDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        add_domain_config_params: models::AddDomainConfigParams {
            domain: "comments.acme.com".to_string(),
            allowed_origins: Some(vec![
                "https://acme.com".to_string(),
                "https://app.acme.com".to_string()
            ]),
            enable_cors: Some(true),
            default_culture: Some("en-US".to_string()),
            public: Some(true),
        },
    };
    let response: AddDomainConfig200Response = add_domain_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
