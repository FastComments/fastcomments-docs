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

Returns: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badges_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badges Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetUserBadges200Response, Error> {
    let params: GetUserBadgesParams = GetUserBadgesParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("user-12345")),
        badge_id: Some(String::from("top-contributor")),
        displayed_on_comments: Some(true),
        limit: Some(50.0),
        skip: Some(0.0),
    };
    let badges: GetUserBadges200Response = get_user_badges(&configuration, params).await?;
    Ok(badges)
}
[inline-code-end]
