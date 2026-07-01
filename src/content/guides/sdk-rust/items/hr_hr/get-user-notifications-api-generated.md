## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| url_id | String | Ne |  |
| page_size | i32 | Ne |  |
| after_id | String | Ne |  |
| include_context | bool | Ne |  |
| after_created_at | i64 | Ne |  |
| unread_only | bool | Ne |  |
| dm_only | bool | Ne |  |
| no_dm | bool | Ne |  |
| include_translations | bool | Ne |  |
| include_tenant_notifications | bool | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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