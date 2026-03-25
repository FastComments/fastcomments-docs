## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| page_size | i32 | 否 |  |
| after_id | String | 否 |  |
| include_context | bool | 否 |  |
| after_created_at | i64 | 否 |  |
| unread_only | bool | 否 |  |
| dm_only | bool | 否 |  |
| no_dm | bool | 否 |  |
| include_translations | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳：[`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_user_notifications 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUserNotificationsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    page_size: Some(25),
    after_id: Some("notif_98765".to_string()),
    include_context: Some(true),
    after_created_at: Some(1_681_500_000i64),
    unread_only: Some(true),
    dm_only: Some(false),
    no_dm: Some(false),
    include_translations: Some(true),
    sso: Some("sso_user_token_ab12".to_string()),
};
let notifications: GetUserNotifications200Response = get_user_notifications(&configuration, params).await?;
[inline-code-end]

---