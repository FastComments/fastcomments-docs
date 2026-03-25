## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page_size | i32 | Nein |  |
| after_id | String | Nein |  |
| include_context | bool | Nein |  |
| after_created_at | i64 | Nein |  |
| unread_only | bool | Nein |  |
| dm_only | bool | Nein |  |
| no_dm | bool | Nein |  |
| include_translations | bool | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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