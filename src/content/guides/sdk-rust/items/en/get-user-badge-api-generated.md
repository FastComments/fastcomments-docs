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
pub async fn fetch_user_badge() -> Result<GetUserBadge200Response, Error> {
    let params: GetUserBadgeParams = GetUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        id: "gold-contributor-1234".to_owned(),
    };
    let response: GetUserBadge200Response = get_user_badge(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
