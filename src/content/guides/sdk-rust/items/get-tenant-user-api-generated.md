## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_tenant_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetTenantUser200Response, Error> {
    let params: GetTenantUserParams = GetTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-42".to_string(),
        include_deleted: Some(false),
        ..Default::default()
    };
    let response: GetTenantUser200Response = get_tenant_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
