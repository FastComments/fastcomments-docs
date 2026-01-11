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
        name: "Top Contributor".to_string(),
        description: Some("Awarded for 100 helpful comments".to_string()),
        icon_url: Some("https://acme-corp.com/assets/badges/top_contributor.png".to_string()),
        criteria: Some("100_helpful_comments".to_string()),
        weight: Some(10),
        active: Some(true),
    },
};

let response: CreateUserBadge200Response = create_user_badge(&configuration, params).await?;
[inline-code-end]
