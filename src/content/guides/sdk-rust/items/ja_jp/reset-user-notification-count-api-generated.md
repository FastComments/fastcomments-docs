## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

## 例

[inline-code-attrs-start title = 'reset_user_notification_count の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reset_user_notification_count() -> Result<ResetUserNotifications200Response, Error> {
    let params: ResetUserNotificationCountParams = ResetUserNotificationCountParams {
        tenant_id: "acme-news-tenant".to_string(),
        sso: Some("user-9876-token".to_string()),
    };
    let response: ResetUserNotifications200Response = reset_user_notification_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---