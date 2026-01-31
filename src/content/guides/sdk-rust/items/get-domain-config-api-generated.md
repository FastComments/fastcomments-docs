## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| domain | String | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_domain_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_domain_config() -> Result<(), Error> {
    let params: GetDomainConfigParams = GetDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain: "news/article".to_string(),
        include_subdomains: Some(true),
    };

    let domain_config: GetDomainConfig200Response = get_domain_config(&configuration, params).await?;

    Ok(())
}
[inline-code-end]
