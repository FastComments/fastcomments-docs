## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vrne: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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