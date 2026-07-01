Activar o desactivar notificaciones para un comentario específico.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenant_id | String | Sí |  |
| notification_id | String | Sí |  |
| opted_in_or_out | String | Sí |  |
| comment_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Ejemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "comment-reply".to_string(),
        opted_in_or_out: "opted_in".to_string(),
        comment_id: "12345".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response = update_user_notification_comment_subscription_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]