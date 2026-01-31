## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | f64 | No |  |

## Response

Returns: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_tenant_packages Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetTenantPackages200Response, Error> {
    let params: GetTenantPackagesParams = GetTenantPackagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let response: GetTenantPackages200Response = get_tenant_packages(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
