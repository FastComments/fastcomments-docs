## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Не |  |
| page_size | i32 | Не |  |
| after_id | String | Не |  |
| include_context | bool | Не |  |
| after_created_at | i64 | Не |  |
| unread_only | bool | Не |  |
| dm_only | bool | Не |  |
| no_dm | bool | Не |  |
| include_translations | bool | Не |  |
| include_tenant_notifications | bool | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## Пример

[inline-code-attrs-start title = 'get_user_notifications Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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