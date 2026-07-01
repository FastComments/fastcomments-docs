## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| user_id | String | Нет |  |
| url_id | String | Нет |  |
| from_comment_id | String | Нет |  |
| viewed | bool | Нет |  |

## Ответ

Возвращает: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("john.doe".to_string()),
        url_id: Some("blog/post-123".to_string()),
        from_comment_id: Some("comment789".to_string()),
        viewed: Some(true),
    };
    let _response = get_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]