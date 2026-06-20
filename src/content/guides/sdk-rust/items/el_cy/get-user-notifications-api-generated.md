## Παράμετροι

| Name | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| url_id | String | Όχι |  |
| page_size | i32 | Όχι |  |
| after_id | String | Όχι |  |
| include_context | bool | Όχι |  |
| after_created_at | i64 | Όχι |  |
| unread_only | bool | Όχι |  |
| dm_only | bool | Όχι |  |
| no_dm | bool | Όχι |  |
| include_translations | bool | Όχι |  |
| include_tenant_notifications | bool | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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