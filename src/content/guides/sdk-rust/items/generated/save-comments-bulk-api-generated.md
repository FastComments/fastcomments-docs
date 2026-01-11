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
async fn example() -> Result<(), Error> {
    let params: SaveCommentsBulkParams = SaveCommentsBulkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: vec![
            models::CreateCommentParams {
                thread_id: "news/article-123".to_string(),
                content: "Great coverage — appreciate the depth.".to_string(),
                author_name: "Jane Doe".to_string(),
                author_email: "jane.doe@acme.com".to_string(),
                parent_id: None,
                mentions: Some(vec![models::CommentUserMentionInfo { user_id: "user-789".to_string(), display_name: "EditorBob".to_string() }]),
                hashtags: Some(vec![models::CommentUserHashTagInfo { tag: "climate".to_string() }]),
            }
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
