## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |

## Response

Returns: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_configs_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_domain_configs Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_domain_configs() -> Result<GetDomainConfigs200Response, Error> {
    let params: GetDomainConfigsParams = GetDomainConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let response: GetDomainConfigs200Response = get_domain_configs(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
