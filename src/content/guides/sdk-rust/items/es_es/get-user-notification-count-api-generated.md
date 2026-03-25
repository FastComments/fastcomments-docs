## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_user_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_user_notification_count() -> Result<(), Error> {
    let params: GetUserNotificationCountParams = GetUserNotificationCountParams {
        tenant_id: String::from("acme-corp-tenant"),
        sso: Some(String::from("sso-jwt-abc123")),
    };
    let _response: GetUserNotificationCount200Response =
        get_user_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---