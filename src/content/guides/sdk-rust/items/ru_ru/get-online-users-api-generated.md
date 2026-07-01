Текущие онлайн‑зрители страницы: люди, чья WebSocket‑сессия подписана на страницу в данный момент. Возвращает `anonCount` + `totalCount` (подписчики на уровне комнаты, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| after_name | String | Нет |  |
| after_user_id | String | Нет |  |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Пример

[inline-code-attrs-start title = 'get_online_users Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]