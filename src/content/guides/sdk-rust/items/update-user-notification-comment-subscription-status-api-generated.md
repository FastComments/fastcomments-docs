Enable or disable notifications for a specific comment.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| notification_id | String | Yes |  |
| opted_in_or_out | String | Yes |  |
| comment_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params: UpdateUserNotificationCommentSubscriptionStatusParams = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "notif-78a3".to_string(),
        opted_in_or_out: "opted_in".to_string(),
        comment_id: "comment-9f8e7d".to_string(),
        sso: Some("sso-token-abcdef123456".to_string()),
    };
    let response: UpdateUserNotificationStatus200Response =
        update_user_notification_comment_subscription_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
