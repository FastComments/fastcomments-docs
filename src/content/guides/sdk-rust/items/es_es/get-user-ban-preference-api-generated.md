## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ApiModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_moderate_get_user_ban_preferences_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'get_user_ban_preference Ejemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserBanPreferenceParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("user123".to_string()),
    };
    let _response = get_user_ban_preference(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---