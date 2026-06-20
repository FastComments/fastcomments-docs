## Параметри

| Name | Type | Required | Description |
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

[inline-code-attrs-start title = 'Приклад search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---