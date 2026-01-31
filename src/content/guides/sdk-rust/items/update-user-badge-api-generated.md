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
let params: UpdateUserBadgeParams = UpdateUserBadgeParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "badge-7a2f".to_string(),
    update_user_badge_params: models::UpdateUserBadgeParams {
        name: Some("Top Contributor".to_string()),
        description: Some("Awarded for 100 helpful comments".to_string()),
        icon_url: Some("https://cdn.acme.com/badges/top-contributor.png".to_string()),
        color: Some("#FFD700".to_string()),
        active: Some(true),
        display_order: Some(1),
    },
};
let updated: UpdateUserBadge200Response = update_user_badge(&configuration, params).await?;
[inline-code-end]
