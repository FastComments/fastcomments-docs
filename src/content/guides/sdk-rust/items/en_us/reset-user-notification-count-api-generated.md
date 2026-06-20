## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Example

[inline-code-attrs-start title = 'reset_user_notification_count Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reset() -> Result<ResetUserNotificationsResponse, Error> {
    let params: ResetUserNotificationCountParams = ResetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("https://sso.acme.com/session/abc123".to_string()),
    };
    let response: ResetUserNotificationsResponse = reset_user_notification_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
