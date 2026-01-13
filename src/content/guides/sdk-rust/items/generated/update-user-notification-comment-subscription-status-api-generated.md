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
async fn example() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params: UpdateUserNotificationCommentSubscriptionStatusParams = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: String::from("acme-corp-tenant"),
        notification_id: String::from("notif-20260112-1234"),
        opted_in_or_out: String::from("opted_in"),
        comment_id: String::from("news/article/2026/updates-12345#cmt-98765"),
        sso: Some(String::from("sso-jwt-abc123def456")),
    };
    let response: UpdateUserNotificationStatus200Response = update_user_notification_comment_subscription_status(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
