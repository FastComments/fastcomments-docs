## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| username_starts_with | String | Hayır |  |
| mention_group_ids | Vec<String> | Hayır |  |
| sso | String | Hayır |  |
| search_section | String | Hayır |  |

## Yanıt

Döndürür: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'search_users Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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