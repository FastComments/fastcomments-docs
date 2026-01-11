## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| delete_comments | bool | No |  |
| comment_delete_mode | String | No |  |

## Response

Returns: [`DeleteSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_sso_user_api_response.rs)

## Example

[inline-code-attrs-start title = 'delete_sso_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_user_example() -> Result<DeleteSsoUserApiResponse, Error> {
    let params = DeleteSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        delete_comments: Some(true),
        comment_delete_mode: Some("permanent".to_string()),
    };
    let response: DeleteSsoUserApiResponse = delete_sso_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
