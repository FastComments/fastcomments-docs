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
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article/2026-01-12/12345/comment-67890"),
        context_user_id: Some(String::from("moderator-42")),
        is_live: Some(false),
    };
    let response: DeleteComment200Response = delete_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
