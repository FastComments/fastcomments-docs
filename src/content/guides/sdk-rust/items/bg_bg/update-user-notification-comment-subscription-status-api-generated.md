Enable or disable notifications for a specific comment.

## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| notification_id | String | Да |  |
| opted_in_or_out | String | Да |  |
| comment_id | String | Да |  |
| sso | String | Не |  |

## Response

Връща: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## Example

[inline-code-attrs-start title = 'Пример за update_user_notification_comment_subscription_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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