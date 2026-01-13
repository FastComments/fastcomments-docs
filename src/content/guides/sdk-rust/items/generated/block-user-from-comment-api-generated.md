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
let params: BlockUserFromCommentParams = BlockUserFromCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "news/article/2026-01-11/comment/7890".to_string(),
    block_from_comment_params: models::BlockFromCommentParams {
        duration_minutes: Some(1440),
        reason: Some("Repeated harassment and hate speech".to_string()),
        block_ip: Some(true),
    },
    user_id: Some("user_9876".to_string()),
    anon_user_id: Some("anon_42".to_string()),
};
let response: BlockFromCommentPublic200Response = block_user_from_comment(&configuration, params).await?;
[inline-code-end]
