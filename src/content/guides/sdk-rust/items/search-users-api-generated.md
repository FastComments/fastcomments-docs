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
async fn run_search() -> Result<(), Error> {
    let params: SearchUsersParams = SearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/2025/politics-election".to_string(),
        username_starts_with: Some("jo".to_string()),
        mention_group_ids: Some(vec!["editors".to_string(), "moderators".to_string()]),
        sso: Some("saml".to_string()),
    };
    let response: SearchUsers200Response = search_users(&configuration, params).await?;
    let _result = response;
    Ok(())
}
[inline-code-end]
