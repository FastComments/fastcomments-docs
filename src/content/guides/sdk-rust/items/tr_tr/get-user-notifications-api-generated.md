## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| page_size | i32 | Hayır |  |
| after_id | String | Hayır |  |
| include_context | bool | Hayır |  |
| after_created_at | i64 | Hayır |  |
| unread_only | bool | Hayır |  |
| dm_only | bool | Hayır |  |
| no_dm | bool | Hayır |  |
| include_translations | bool | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_user_notifications Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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