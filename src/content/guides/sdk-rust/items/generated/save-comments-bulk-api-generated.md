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
    let params = SaveCommentsBulkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: vec![
            models::CreateCommentParams {
                content: "Great reporting — clear sources and balanced view.".to_string(),
                thread_slug: "news/world/uk-election-coverage".to_string(),
                author_name: "Jane Doe".to_string(),
                author_email: "jane.doe@example.com".to_string(),
                ..Default::default()
            },
            models::CreateCommentParams {
                content: "I disagree with the conclusions; more data needed.".to_string(),
                thread_slug: "news/world/uk-election-coverage".to_string(),
                author_name: "Carlos Ruiz".to_string(),
                author_email: "carlos.ruiz@example.org".to_string(),
                ..Default::default()
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
