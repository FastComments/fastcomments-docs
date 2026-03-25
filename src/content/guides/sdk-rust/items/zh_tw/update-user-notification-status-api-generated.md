---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| notification_id | String | 是 |  |
| new_status | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## 範例

[inline-code-attrs-start title = 'update_user_notification_status 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params: UpdateUserNotificationStatusParams = UpdateUserNotificationStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "notif-2026-04-01-7f3b".to_string(),
        new_status: "read".to_string(),
        sso: Some("sso-session-abcdef123456".to_string()),
    };
    let resp: UpdateUserNotificationStatus200Response =
        update_user_notification_status(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---