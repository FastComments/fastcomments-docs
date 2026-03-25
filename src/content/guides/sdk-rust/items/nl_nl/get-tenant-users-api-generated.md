## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| skip | f64 | Nee |  |

## Antwoord

Retourneert: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_tenant_users Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tenant_users() -> Result<(), Error> {
    let params: GetTenantUsersParams = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let users: GetTenantUsers200Response = get_tenant_users(&configuration, params).await?;
    let _users = users;
    Ok(())
}
[inline-code-end]