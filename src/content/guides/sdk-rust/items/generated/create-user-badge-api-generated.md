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
async fn example_create_user_badge() -> Result<CreateUserBadge200Response, Error> {
    let params: CreateUserBadgeParams = CreateUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_user_badge_params: models::CreateUserBadgeParams {
            user_id: "user-789".to_string(),
            badge_key: "top_contributor".to_string(),
            title: Some("Top Contributor".to_string()),
            description: Some("Awarded for exceptional contributions to the community".to_string()),
            image_url: Some("https://cdn.acme.com/badges/top_contributor.png".to_string()),
            active: Some(true),
            expires_at: None,
        },
    };
    let response: CreateUserBadge200Response = create_user_badge(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
