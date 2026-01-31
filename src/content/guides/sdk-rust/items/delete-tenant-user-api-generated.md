## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| delete_comments | String | No |  |
| comment_delete_mode | String | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_tenant_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: DeleteTenantUserParams = DeleteTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        delete_comments: Some("true".to_string()),
        comment_delete_mode: Some("soft".to_string()),
    };
    let _response: FlagCommentPublic200Response = delete_tenant_user(configuration, params).await?;
    Ok(())
}
[inline-code-end]
