## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| page_size | i32 | Ні |  |
| after_id | String | Ні |  |
| include_context | bool | Ні |  |
| after_created_at | i64 | Ні |  |
| unread_only | bool | Ні |  |
| dm_only | bool | Ні |  |
| no_dm | bool | Ні |  |
| include_translations | bool | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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