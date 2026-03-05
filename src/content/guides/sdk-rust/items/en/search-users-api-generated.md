## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| username_starts_with | String | No |  |
| mention_group_ids | Vec<String> | No |  |
| sso | String | No |  |

## Response

Returns: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_200_response.rs)

## Example

[inline-code-attrs-start title = 'search_users Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn perform_user_search() -> Result<SearchUsers200Response, Error> {
    let params: SearchUsersParams = SearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-2026".to_string(),
        username_starts_with: Some("jo".to_string()),
        mention_group_ids: Some(vec!["editors".to_string(), "authors".to_string()]),
        sso: Some("saml".to_string()),
    };
    let response: SearchUsers200Response = search_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
