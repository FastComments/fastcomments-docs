## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| send_email | String | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_moderator Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteModeratorParams = DeleteModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-7b9e4f".to_string(),
        send_email: Some("true".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
