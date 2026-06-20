---
Habilitar o deshabilitar las notificaciones para un comentario específico.

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| notification_id | String | Sí |  |
| opted_in_or_out | String | Sí |  |
| comment_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de update_user_notification_comment_subscription_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationCommentSubscriptionStatusResponse, Error> {
    let params: UpdateUserNotificationCommentSubscriptionStatusParams = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "news/winter-2026-update".to_string(),
        opted_in_or_out: "opted_in".to_string(),
        comment_id: "article-42-comment-7".to_string(),
        sso: Some("user-123|eyJhbGciOi...".to_string()),
    };
    let response: UpdateUserNotificationCommentSubscriptionStatusResponse =
        update_user_notification_comment_subscription_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---