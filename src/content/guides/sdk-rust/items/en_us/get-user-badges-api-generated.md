## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| badge_id | String | No |  |
| displayed_on_comments | bool | No |  |
| limit | f64 | No |  |
| skip | f64 | No |  |

## Response

Returns: [`ApiGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badges_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badges Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
