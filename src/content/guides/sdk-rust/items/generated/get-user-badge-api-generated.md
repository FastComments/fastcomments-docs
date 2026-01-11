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
async fn fetch_user_badge() -> Result<GetUserBadge200Response, Error> {
    let maybe_tenant: Option<String> = Some("acme-corp-tenant".to_string());
    let params: GetUserBadgeParams = GetUserBadgeParams {
        tenant_id: maybe_tenant.unwrap(),
        id: "top-commenter-2025".to_string(),
    };
    let badge: GetUserBadge200Response = get_user_badge(&configuration, params).await?;
    Ok(badge)
}
[inline-code-end]
