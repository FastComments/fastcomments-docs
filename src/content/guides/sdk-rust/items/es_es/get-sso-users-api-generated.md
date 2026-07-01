## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|--------------|-------------|
| tenant_id | String | Sí |  |
| skip | i32 | No |  |

## Respuesta

Devuelve: [`GetSsoUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_sso_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetSsoUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10),
    };
    let _response = get_sso_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---