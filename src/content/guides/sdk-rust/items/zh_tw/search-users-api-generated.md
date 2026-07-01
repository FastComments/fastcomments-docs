## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| username_starts_with | String | 否 |  |
| mention_group_ids | Vec<String> | 否 |  |
| sso | String | 否 |  |
| search_section | String | 否 |  |

## 回應

返回: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## 範例

[inline-code-attrs-start title = 'search_users 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---