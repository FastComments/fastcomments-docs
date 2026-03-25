## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Yes |  |
| update_comments | String | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_tenant_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_tenant_user(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateTenantUserParams = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9f".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            email: Some("jane.doe@acme.com".to_string()),
            display_name: Some("Jane Doe".to_string()),
            username: Some("jdoe".to_string()),
            role: Some("moderator".to_string()),
        },
        update_comments: Some("Promoted to moderator to handle flagged comments".to_string()),
    };
    let response = update_tenant_user(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
