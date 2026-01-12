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
async fn run() -> Result<UnBlockCommentPublic200Response, Error> {
    let params = UnBlockCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/2026-01-12/comment-98765".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams {
            unblocked_by: "moderator-42".to_string(),
            reason: Some("mistakenly flagged as spam".to_string()),
            timestamp: Some("2026-01-12T10:00:00Z".to_string()),
        },
        sso: Some("sso-jwt-7f3b".to_string()),
    };
    let response: UnBlockCommentPublic200Response = un_block_comment_public(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
