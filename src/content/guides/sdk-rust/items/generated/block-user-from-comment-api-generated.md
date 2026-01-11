## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| block_from_comment_params | models::BlockFromCommentParams | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Response

Returns: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_from_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'block_user_from_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_block() -> Result<(), Error> {
    let params: BlockUserFromCommentParams = BlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2025/11/21/comment-12345".to_string(),
        block_from_comment_params: models::BlockFromCommentParams {
            reason: Some("Repeated abusive language".to_string()),
            duration_hours: Some(168),
            notify: Some(true),
        },
        user_id: Some("user-9876".to_string()),
        anon_user_id: None,
    };

    let response: BlockFromCommentPublic200Response = block_user_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
