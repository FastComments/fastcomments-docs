Benachrichtigungen für einen bestimmten Kommentar aktivieren oder deaktivieren.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| notification_id | String | Ja |  |
| opted_in_or_out | String | Ja |  |
| comment_id | String | Ja |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## Beispiel

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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