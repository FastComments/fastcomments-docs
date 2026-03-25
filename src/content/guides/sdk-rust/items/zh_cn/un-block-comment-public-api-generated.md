## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/un_block_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'un_block_comment_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<UnBlockCommentPublic200Response, Error> {
    let params: UnBlockCommentPublicParams = UnBlockCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/2026-03-25/comment-98765".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams {
            unblocked_by: "moderator@acme.com".to_string(),
            reason: "Reviewed and determined not to be spam".to_string(),
        },
        sso: Some("sso-session-token-abc123".to_string()),
    };
    let response: UnBlockCommentPublic200Response = un_block_comment_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---