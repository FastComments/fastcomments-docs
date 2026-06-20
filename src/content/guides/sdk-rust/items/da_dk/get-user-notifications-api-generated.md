## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Nej |  |
| page_size | i32 | Nej |  |
| after_id | String | Nej |  |
| include_context | bool | Nej |  |
| after_created_at | i64 | Nej |  |
| unread_only | bool | Nej |  |
| dm_only | bool | Nej |  |
| no_dm | bool | Nej |  |
| include_translations | bool | Nej |  |
| include_tenant_notifications | bool | Nej |  |
| sso | String | Nej |  |

## Respons

Returnerer: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---