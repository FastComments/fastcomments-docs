## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| user_id | String | Нет |  |
| badge_id | String | Нет |  |
| displayed_on_comments | bool | Нет |  |
| limit | f64 | Нет |  |
| skip | f64 | Нет |  |

## Ответ

Возвращает: [`ApiGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badges_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_user_badges'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_user_badges() -> Result<ApiGetUserBadgesResponse, Error> {
    let params: GetUserBadgesParams = GetUserBadgesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user_7890".to_string()),
        badge_id: Some("top-commenter".to_string()),
        displayed_on_comments: Some(true),
        limit: Some(25.0),
        skip: Some(0.0),
    };
    let response: ApiGetUserBadgesResponse = get_user_badges(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---