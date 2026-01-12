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
async fn example_get_domain_config() -> Result<GetDomainConfig200Response, Error> {
    let params: GetDomainConfigParams = GetDomainConfigParams {
        tenant_id: String::from("acme-corp-tenant"),
        domain: String::from("news/world"),
        include_subdomains: Some(true),
    };
    let config_response: GetDomainConfig200Response = get_domain_config(&configuration, params).await?;
    Ok(config_response)
}
[inline-code-end]
