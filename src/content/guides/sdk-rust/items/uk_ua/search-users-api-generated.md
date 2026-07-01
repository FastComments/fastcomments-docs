## Параметри

| Ім'я | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| username_starts_with | String | Ні |  |
| mention_group_ids | Vec<String> | Ні |  |
| sso | String | Ні |  |
| search_section | String | Ні |  |

## Відповідь

Повертає: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## Приклад

[inline-code-attrs-start title = 'search_users Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_search() -> Result<(), Error> {
    let params = SearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        username_starts_with: Some("john".to_string()),
        mention_group_ids: Some(vec!["group1".to_string(), "group2".to_string()]),
        sso: Some("sso-provider".to_string()),
        search_section: Some("comments".to_string()),
    };
    let _result: SearchUsersResult = search_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]