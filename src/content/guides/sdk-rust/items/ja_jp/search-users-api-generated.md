## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |
| username_starts_with | String | いいえ |  |
| mention_group_ids | Vec<String> | いいえ |  |
| sso | String | いいえ |  |
| search_section | String | いいえ |  |

## レスポンス

戻り値: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## 例

[inline-code-attrs-start title = 'search_users の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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