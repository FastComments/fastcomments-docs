## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| un_block_from_comment_params | models::UnBlockFromCommentParams | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/un_block_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'un_block_user_from_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UnBlockUserFromCommentParams = UnBlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/comment-456".to_string(),
        un_block_from_comment_params: models::UnBlockFromCommentParams::default(),
        user_id: Some("user-123".to_string()),
        anon_user_id: None,
    };
    let response: UnBlockCommentPublic200Response = un_block_user_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
