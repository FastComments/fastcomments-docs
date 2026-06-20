## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Nie |  |
| page_size | i32 | Nie |  |
| after_id | String | Nie |  |
| include_context | bool | Nie |  |
| after_created_at | i64 | Nie |  |
| unread_only | bool | Nie |  |
| dm_only | bool | Nie |  |
| no_dm | bool | Nie |  |
| include_translations | bool | Nie |  |
| include_tenant_notifications | bool | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications() -> Result<GetMyNotificationsResponse, Error> {
    let params: GetUserNotificationsParams = GetUserNotificationsParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: Some(String::from("news/product-launch")),
        page_size: Some(25),
        after_id: Some(String::from("notif_1024")),
        include_context: Some(true),
        after_created_at: Some(1_676_000_000i64),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(false),
        include_translations: Some(true),
        include_tenant_notifications: Some(false),
        sso: Some(String::from("sso_token_abc123")),
    };
    let notifications: GetMyNotificationsResponse = get_user_notifications(&configuration, params).await?;
    Ok(notifications)
}
[inline-code-end]