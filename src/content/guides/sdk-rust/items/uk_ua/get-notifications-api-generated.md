## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| user_id | String | Ні |  |
| url_id | String | Ні |  |
| from_comment_id | String | Ні |  |
| viewed | bool | Ні |  |
| skip | f64 | Ні |  |

## Відповідь

Повертає: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-9a7b".to_string()),
        url_id: Some("news/article/launch-announcement".to_string()),
        from_comment_id: Some("cmt-1024".to_string()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let notifications: GetNotificationsResponse = get_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]