## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_moderator_body | models::UpdateModeratorBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_moderator Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<models::FlagCommentPublic200Response, Error> {
    let params: UpdateModeratorParams = UpdateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-123".to_string(),
        update_moderator_body: models::UpdateModeratorBody {
            display_name: Some("Jane Doe".to_string()),
            email: Some("jane.doe@acme-corp.com".to_string()),
            active: Some(true),
            roles: Some(vec!["site_moderator".to_string(), "content_reviewer".to_string()]),
            note: Some("Senior moderator for news and opinion".to_string()),
        },
    };
    let resp: models::FlagCommentPublic200Response = update_moderator(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]
