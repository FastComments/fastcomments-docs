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
async fn example_add_domain() -> Result<(), Error> {
    let params: AddDomainConfigParams = AddDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        add_domain_config_params: models::AddDomainConfigParams {
            domain: "comments.acme.com".to_string(),
            site_path: "/news/article".to_string(),
            enabled: Some(true),
            allow_subdomains: Some(true),
            include_paths: Some(vec!["/news/*".to_string(), "/blog/*".to_string()]),
        },
    };

    let resp: AddDomainConfig200Response = add_domain_config(&configuration, params).await?;
    println!("{:#?}", resp);
    Ok(())
}
[inline-code-end]
