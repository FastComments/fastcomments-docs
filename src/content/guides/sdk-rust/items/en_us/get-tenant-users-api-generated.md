## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | f64 | No |  |

## Response

Returns: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_response.rs)

## Example

[inline-code-attrs-start title = 'get_tenant_users Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetTenantUsersParams = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetTenantUsersResponse = get_tenant_users(&configuration, params).await?;
    let _users: GetTenantUsersResponse = response;
    Ok(())
}
[inline-code-end]
