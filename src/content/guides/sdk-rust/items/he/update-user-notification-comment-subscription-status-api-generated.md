---
הפעל או השבת התראות עבור תגובה ספציפית.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| notification_id | String | כן |  |
| opted_in_or_out | String | כן |  |
| comment_id | String | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_user_notification_comment_subscription_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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