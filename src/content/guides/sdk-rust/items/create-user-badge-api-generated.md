## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_user_badge_params | models::CreateUserBadgeParams | Yes |  |

## Response

Returns: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_user_badge_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_user_badge Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateUserBadgeParams = CreateUserBadgeParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_user_badge_params: models::CreateUserBadgeParams {
        user_id: "user-987".to_string(),
        badge_id: "veteran-commenter".to_string(),
        awarded_by: Some("moderator-42".to_string()),
        reason: Some("Consistent high-quality contributions".to_string()),
    },
};
let response: CreateUserBadge200Response = create_user_badge(&configuration, params).await?;
[inline-code-end]
