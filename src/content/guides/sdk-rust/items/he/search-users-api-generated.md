## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | כן |  |
| username_starts_with | String | לא |  |
| mention_group_ids | Vec<String> | לא |  |
| sso | String | לא |  |
| search_section | String | לא |  |

## תגובה

מחזיר: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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