## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| notification_id | String | はい |  |
| new_status | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_response.rs)

## 例

[inline-code-attrs-start title = 'update_user_notification_status の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<UpdateUserNotificationStatusResponse, Error> {
    let params: UpdateUserNotificationStatusParams = UpdateUserNotificationStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "notifications/8472".to_string(),
        new_status: "dismissed".to_string(),
        sso: Some("sso-user-98765-token".to_string()),
    };
    let response: UpdateUserNotificationStatusResponse =
        update_user_notification_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---