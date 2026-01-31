## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Yes |  |
| sso | String | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/un_block_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'un_block_comment_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unblock_comment() -> Result<(), Error> {
    let public_block_from_comment_params: models::PublicBlockFromCommentParams = Default::default();
    let params = UnBlockCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-1234/comment-5678".to_string(),
        public_block_from_comment_params,
        sso: Some("sso-token-4f3b2a".to_string()),
    };
    let unblocked: UnBlockCommentPublic200Response = un_block_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
