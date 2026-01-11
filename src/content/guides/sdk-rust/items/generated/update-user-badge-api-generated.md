## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_user_badge_params | models::UpdateUserBadgeParams | Yes |  |

## Response

Returns: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_badge_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_user_badge Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<UpdateUserBadge200Response, Error> {
    let params: UpdateUserBadgeParams = UpdateUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-789".to_string(),
        update_user_badge_params: models::UpdateUserBadgeParams {
            badge_id: "trusted-contributor".to_string(),
            label: Some("Trusted Contributor".to_string()),
            description: Some("Awarded for sustained high-quality contributions".to_string()),
            is_active: Some(true),
            awarded_at: Some("2025-01-15T12:00:00Z".to_string()),
            expires_at: None,
        },
    };
    let resp: UpdateUserBadge200Response = update_user_badge(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]
