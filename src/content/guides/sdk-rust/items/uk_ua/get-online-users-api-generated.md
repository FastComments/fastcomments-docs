Поточні онлайн‑глядачі сторінки: користувачі, чия сесія WebSocket підписана на сторінку зараз.  
Повертає anonCount + totalCount (підписники по всій кімнаті, включаючи анонімних глядачів, які не перераховуються).

## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| after_name | String | Ні |  |
| after_user_id | String | Ні |  |

## Відповідь

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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