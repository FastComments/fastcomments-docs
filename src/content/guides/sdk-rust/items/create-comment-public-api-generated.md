## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| comment_data | models::CommentData | Yes |  |
| session_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_comment_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = CreateCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/2026/01/fast-rust-api".to_string(),
        broadcast_id: "global-feed".to_string(),
        comment_data: models::CommentData {
            content: "Insightful coverage — thanks!".to_string(),
            author_name: Some("Jane Doe".to_string()),
            email: Some("jane.doe@example.com".to_string()),
            mentions: vec![
                models::CommentUserMentionInfo { user_id: "user-123".to_string(), display: "john_smith".to_string() }
            ],
            hashtags: vec![
                models::CommentUserHashTagInfo { tag: "rust".to_string() }
            ],
            parent_id: None,
        },
        session_id: Some("sess-9a8b7c".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let response: CreateCommentPublic200Response = create_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
