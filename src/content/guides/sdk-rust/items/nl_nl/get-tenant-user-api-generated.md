## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Retourneert: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_tenant_user Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tenant_user() -> Result<GetTenantUserResponse, Error> {
    let params: GetTenantUserParams = GetTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9a2".to_string(),
        include_profile: Some(true),
    };
    let response: GetTenantUserResponse = get_tenant_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---