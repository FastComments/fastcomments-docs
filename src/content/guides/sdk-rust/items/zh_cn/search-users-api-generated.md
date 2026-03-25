## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| username_starts_with | String | 否 |  |
| mention_group_ids | Vec<String> | 否 |  |
| sso | String | 否 |  |
| search_section | String | 否 |  |

## 响应

Returns: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_200_response.rs)

## 示例

[inline-code-attrs-start title = 'search_users 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_search() -> Result<SearchUsers200Response, Error> {
    let params: SearchUsersParams = SearchUsersParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/article-2026-03-25-launch"),
        username_starts_with: Some(String::from("alex")),
        mention_group_ids: Some(vec![String::from("team-marketing"), String::from("team-product")]),
        sso: Some(String::from("okta")),
        search_section: Some(String::from("comments")),
    };
    let search_result: SearchUsers200Response = search_users(&configuration, params).await?;
    Ok(search_result)
}
[inline-code-end]

---