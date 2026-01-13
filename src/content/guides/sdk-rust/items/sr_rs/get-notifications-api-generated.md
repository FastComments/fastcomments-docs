## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| user_id | String | Не |  |
| url_id | String | Не |  |
| from_comment_id | String | Не |  |
| viewed | bool | Не |  |
| skip | f64 | Не |  |

## Одговор

Враћа: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_200_response.rs)

## Пример

[inline-code-attrs-start title = 'get_notifications Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-1234".to_string()),
        url_id: Some("news/politics/article-2026-01-12".to_string()),
        from_comment_id: Some("cmt-98765".to_string()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let notifications: GetNotifications200Response = get_notifications(&configuration, params).await?;
    let _ = notifications;
    Ok(())
}
[inline-code-end]

---