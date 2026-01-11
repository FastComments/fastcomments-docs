## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| domain | String | Yes |  |

## Response

Returns: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_domain_config_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_domain_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let domain_opt: Option<String> = Some("blog.acme.com".to_string());
let params: DeleteDomainConfigParams = DeleteDomainConfigParams {
    tenant_id: "acme-corp-tenant".to_string(),
    domain: domain_opt.unwrap(),
};
let response: DeleteDomainConfig200Response = delete_domain_config(&configuration, params).await?;
[inline-code-end]
