Enable or disable notifications for a specific comment.

## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| notification_id | String | 예 |  |
| opted_in_or_out | String | 예 |  |
| comment_id | String | 예 |  |
| sso | String | 아니오 |  |

## Response

Returns: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## Example

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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