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
async fn update_badge_example() -> Result<(), Error> {
    let params: UpdateUserBadgeParams = UpdateUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-top-commenter-001".to_string(),
        update_user_badge_params: models::UpdateUserBadgeParams {
            title: Some("Top Commenter".to_string()),
            description: Some("Awarded for 100+ comments across news and opinion sections".to_string()),
            enabled: Some(true),
            icon_url: Some("https://cdn.acme-corp.com/assets/badges/top-commenter.png".to_string()),
            color: Some("#FFD700".to_string()),
            min_comments: Some(100),
        },
    };
    let response: UpdateUserBadge200Response = update_user_badge(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
