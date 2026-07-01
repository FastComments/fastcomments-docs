Enable or disable notifications for a specific comment.

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| notification_id | String | Sim |  |
| opted_in_or_out | String | Sim |  |
| comment_id | String | Sim |  |
| sso | String | Não |  |

## Resposta

Returns: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## Exemplo

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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