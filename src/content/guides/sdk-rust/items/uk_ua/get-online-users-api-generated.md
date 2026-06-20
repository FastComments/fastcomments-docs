Користувачі, які зараз онлайн на сторінці: це люди, чиї websocket-сесії зараз підписані на цю сторінку.
Повертає anonCount + totalCount (підписники по всій кімнаті, включно з анонімними глядачами, яких ми не перераховуємо).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| after_name | String | Ні |  |
| after_user_id | String | Ні |  |

## Відповідь

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_online_users() -> Result<PageUsersOnlineResponse, Error> {
    let params: GetOnlineUsersParams = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/article-2026".to_string(),
        after_name: Some("jane.doe".to_string()),
        after_user_id: Some("user_98765".to_string()),
    };
    let response: PageUsersOnlineResponse = get_online_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---