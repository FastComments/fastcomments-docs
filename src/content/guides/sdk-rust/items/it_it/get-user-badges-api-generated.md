## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| user_id | String | No |  |
| badge_id | String | No |  |
| displayed_on_comments | bool | No |  |
| limit | f64 | No |  |
| skip | f64 | No |  |

## Risposta

Restituisce: [`ApiGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badges_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio get_user_badges'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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