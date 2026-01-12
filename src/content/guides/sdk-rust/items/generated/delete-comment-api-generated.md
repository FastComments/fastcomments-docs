## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| context_user_id | String | No |  |
| is_live | bool | No |  |

## Response

Returns: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<DeleteComment200Response, Error> {
    let params: DeleteCommentParams = DeleteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-8a7f3c2b-4d9e-4f1a-9c2b-1e6f3a9b2d4c".to_string(),
        context_user_id: Some("user-3521".to_string()),
        is_live: Some(true),
    };
    let response: DeleteComment200Response = delete_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
