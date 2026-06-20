## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenant_id | String | Ja |  |
| user_id | String | Nee |  |
| badge_id | String | Nee |  |
| displayed_on_comments | bool | Nee |  |
| limit | f64 | Nee |  |
| skip | f64 | Nee |  |

## Respons

Retourneert: [`ApiGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badges_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_user_badges Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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