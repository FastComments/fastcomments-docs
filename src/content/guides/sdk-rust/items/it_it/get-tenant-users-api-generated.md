## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| skip | f64 | No |  |

## Risposta

Restituisce: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_tenant_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_tenant_users() -> Result<GetTenantUsers200Response, Error> {
    let params: GetTenantUsersParams = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let response: GetTenantUsers200Response = get_tenant_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---