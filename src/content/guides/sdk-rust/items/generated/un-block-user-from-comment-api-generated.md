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
let params: UnBlockUserFromCommentParams = UnBlockUserFromCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "news/article/2025/interesting-story-12345".to_string(),
    un_block_from_comment_params: models::UnBlockFromCommentParams {
        reason: Some("Appeal accepted; user reinstated".to_string()),
        unblocked_by: Some("moderator-42".to_string()),
    },
    user_id: Some("user-9876".to_string()),
    anon_user_id: Some("anon-1234".to_string()),
};
let response: UnBlockCommentPublic200Response = un_block_user_from_comment(&configuration, params).await?;
[inline-code-end]
