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
async fn update_user_example() -> Result<(), Error> {
    let params: UpdateTenantUserParams = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            email: "jane.doe@acme.com".to_string(),
            display_name: Some("Jane Doe".to_string()),
            role: Some("moderator".to_string()),
            suspended: Some(false),
        },
        update_comments: Some("Promoted to moderator and granted moderation privileges".to_string()),
    };
    let response: FlagCommentPublic200Response = update_tenant_user(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]
