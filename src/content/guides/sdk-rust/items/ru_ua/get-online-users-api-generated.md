Currently-online viewers of a page: люди, чья websocket‑сессия в данный момент подписана на страницу.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate). → Возвращает anonCount + totalCount (подписчики всей комнаты, включая анонимных зрителей, которые мы не перечисляем).

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Response

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Example

[inline-code-attrs-start title = 'Пример get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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