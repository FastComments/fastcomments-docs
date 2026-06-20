## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| user_id | String | Ne |  |
| badge_id | String | Ne |  |
| displayed_on_comments | bool | Ne |  |
| limit | f64 | Ne |  |
| skip | f64 | Ne |  |

## Odgovor

Vrne: [`ApiGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badges_response.rs)

## Primer

[inline-code-attrs-start title = 'get_user_badges Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_user_badges() -> Result<ApiGetUserBadgesResponse, Error> {
    let params: GetUserBadgesParams = GetUserBadgesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user_7890".to_string()),
        badge_id: Some("top-commenter".to_string()),
        displayed_on_comments: Some(true),
        limit: Some(25.0),
        skip: Some(0.0),
    };
    let response: ApiGetUserBadgesResponse = get_user_badges(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---