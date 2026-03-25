## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| page_size | i32 | Ne |  |
| after_id | String | Ne |  |
| include_context | bool | Ne |  |
| after_created_at | i64 | Ne |  |
| unread_only | bool | Ne |  |
| dm_only | bool | Ne |  |
| no_dm | bool | Ne |  |
| include_translations | bool | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## Primer

[inline-code-attrs-start title = 'get_user_notifications Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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