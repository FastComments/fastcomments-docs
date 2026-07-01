## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
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
async fn example() -> Result<(), Error> {
    let params = GetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: Some("news/article".to_string()),
        page_size: Some(20),
        after_id: None,
        include_context: Some(true),
        after_created_at: None,
        unread_only: Some(false),
        dm_only: Some(false),
        no_dm: Some(true),
        include_translations: Some(false),
        include_tenant_notifications: Some(true),
        sso: None,
    };
    let _resp = get_user_notifications(&config, params).await?;
    Ok(())
}
[inline-code-end]