Включване или изключване на известия за конкретен коментар.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| notification_id | String | Yes |  |
| opted_in_or_out | String | Yes |  |
| comment_id | String | Yes |  |
| sso | String | No |  |

## Отговор

Връща: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## Пример

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_user_notification_comment_subscription_status() -> Result<(), Error> {
    let params: UpdateUserNotificationCommentSubscriptionStatusParams = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "notif-2026-03-25-4f2b".to_string(),
        opted_in_or_out: "opted_out".to_string(),
        comment_id: "cmt-98a7b6c5d4".to_string(),
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: UpdateUserNotificationStatus200Response =
        update_user_notification_comment_subscription_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---