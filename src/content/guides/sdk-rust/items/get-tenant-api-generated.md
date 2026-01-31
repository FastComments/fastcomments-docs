## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_tenant Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenant() -> Result<(), Error> {
    let params = GetTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        include_domains: Some(true),
    };
    let tenant: GetTenant200Response = get_tenant(&configuration, params).await?;
    println!("{:#?}", tenant);
    Ok(())
}
[inline-code-end]
