## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| page_size | i32 | לא |  |
| after_id | String | לא |  |
| include_context | bool | לא |  |
| after_created_at | i64 | לא |  |
| unread_only | bool | לא |  |
| dm_only | bool | לא |  |
| no_dm | bool | לא |  |
| include_translations | bool | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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