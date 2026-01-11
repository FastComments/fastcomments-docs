## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Yes |  |
| sso | String | No |  |

## Response

Returns: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_from_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'block_from_comment_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: BlockFromCommentPublicParams = BlockFromCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2025-11-21-cmt-987654".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams::default(),
        sso: Some("sso-jwt-abcdef123456".to_string()),
    };
    let response: BlockFromCommentPublic200Response = block_from_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
