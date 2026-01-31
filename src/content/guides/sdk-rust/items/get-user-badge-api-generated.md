## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badge Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badge() -> Result<GetUserBadge200Response, Error> {
    let params: GetUserBadgeParams = GetUserBadgeParams {
        tenant_id: "acme-news-tenant".to_string(),
        id: "editorial-badge-42".to_string(),
    };
    let badge_response: GetUserBadge200Response = get_user_badge(&configuration, params).await?;
    Ok(badge_response)
}
[inline-code-end]
