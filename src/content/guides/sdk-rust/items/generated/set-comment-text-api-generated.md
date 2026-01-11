## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Response

Returns: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

## Example

[inline-code-attrs-start title = 'set_comment_text Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: SetCommentTextParams = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345/comment-6789".to_string(),
        broadcast_id: "news/nytimes/frontpage".to_string(),
        comment_text_update_request: models::CommentTextUpdateRequest {
            text: "Updated: the event has been postponed until further notice.".to_string(),
            mentions: vec![
                models::CommentUserMentionInfo {
                    user_id: "user-42".to_string(),
                    display_name: "Jane Doe".to_string(),
                }
            ],
            hashtags: vec![
                models::CommentUserHashTagInfo { tag: "update".to_string() }
            ],
        },
        edit_key: Some("editkey-0a1b2c3d4e".to_string()),
        sso: Some("sso-token-9f8e7d6c".to_string()),
    };
    let response: SetCommentText200Response = set_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
