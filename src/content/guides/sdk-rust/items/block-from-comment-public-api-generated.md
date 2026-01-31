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
let params: BlockFromCommentPublicParams = BlockFromCommentPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "news/article-9876543210-cmt42".to_string(),
    public_block_from_comment_params: models::PublicBlockFromCommentParams {
        reason: "spam".to_string(),
        duration_minutes: Some(60),
        notify_user: Some(true),
        moderator_note: Some("User repeatedly posted affiliate links".to_string()),
    },
    sso: Some("sso-session-6a1b2c".to_string()),
};

let response: BlockFromCommentPublic200Response = block_from_comment_public(&configuration, params).await?;
[inline-code-end]
