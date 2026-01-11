## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

## Example

[inline-code-attrs-start title = 'reset_user_notification_count Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: ResetUserNotificationCountParams = ResetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature".to_string()),
    };
    let resp: ResetUserNotifications200Response = reset_user_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
