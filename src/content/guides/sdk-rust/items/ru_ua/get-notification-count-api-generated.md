---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| user_id | String | Нет |  |
| url_id | String | Нет |  |
| from_comment_id | String | Нет |  |
| viewed | bool | Нет |  |

## Ответ

Возвращает: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetNotificationCountParams = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-67890".to_string()),
        url_id: Some("news/2026/03/25/election-updates".to_string()),
        from_comment_id: Some("cmt_42".to_string()),
        viewed: Some(false),
    };
    let response: GetNotificationCount200Response = get_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---