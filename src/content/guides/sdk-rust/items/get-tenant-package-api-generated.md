## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_package_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_tenant_package Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let tenant_alias: Option<String> = Some("acme-corp-tenant".to_string());
    let package_id: String = "premium-moderation".to_string();
    let params: GetTenantPackageParams = GetTenantPackageParams {
        tenant_id: tenant_alias.clone().unwrap(),
        id: package_id,
    };
    let package_response: GetTenantPackage200Response = get_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
