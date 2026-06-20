## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| username_starts_with | String | Нет |  |
| mention_group_ids | Vec<String> | Нет |  |
| sso | String | Нет |  |
| search_section | String | Нет |  |

## Ответ

Возвращает: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## Пример

[inline-code-attrs-start title = 'Пример search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_users() -> Result<(), Error> {
    let params: SearchUsersParams = SearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-2026-06".to_string(),
        username_starts_with: Some("jo".to_string()),
        mention_group_ids: Some(vec![
            "group-moderators".to_string(),
            "group-editors".to_string(),
        ]),
        sso: Some("google".to_string()),
        search_section: Some("comments".to_string()),
    };

    let result: SearchUsersResult = search_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]