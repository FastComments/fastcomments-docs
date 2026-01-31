## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_comment_params | Vec<models::CreateCommentParams> | Yes |  |
| is_live | bool | No |  |
| do_spam_check | bool | No |  |
| send_emails | bool | No |  |
| populate_notifications | bool | No |  |

## Response

Returns: `Vec<models::SaveComment200Response>`

## Example

[inline-code-attrs-start title = 'save_comments_bulk Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: SaveCommentsBulkParams = SaveCommentsBulkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: vec![
            models::CreateCommentParams {
                content: "Great breakdown of the deployment strategy — very helpful!".to_string(),
                thread_id: "news/article-2026-01-13".to_string(),
                author_name: "Alice Johnson".to_string(),
                author_email: Some("alice.johnson@acme.com".to_string()),
                mentions: Some(vec![
                    models::CommentUserMentionInfo { user_id: "user-789".to_string(), display_name: Some("DevGuru".to_string()) }
                ]),
                hashtags: Some(vec![
                    models::CommentUserHashTagInfo { tag: "devops".to_string() }
                ]),
            },
            models::CreateCommentParams {
                content: "I disagree with the recommendation on cache invalidation.".to_string(),
                thread_id: "news/article-2026-01-13".to_string(),
                author_name: "Bob Lee".to_string(),
                author_email: None,
                mentions: None,
                hashtags: None,
            },
        ],
        is_live: Some(true),
        do_spam_check: Some(true),
        send_emails: Some(false),
        populate_notifications: Some(true),
    };
    let responses: Vec<models::SaveComment200Response> = save_comments_bulk(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
