## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| create_comment_params | Vec<models::CreateCommentParams> | हाँ |  |
| is_live | bool | नहीं |  |
| do_spam_check | bool | नहीं |  |
| send_emails | bool | नहीं |  |
| populate_notifications | bool | नहीं |  |

## प्रतिक्रिया

रिटर्न: `Vec<models::SaveCommentsBulkResponse>`

## उदाहरण

[inline-code-attrs-start title = 'save_comments_bulk उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params: SaveCommentsBulkParams = SaveCommentsBulkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: vec![
            models::CreateCommentParams::default(),
            models::CreateCommentParams::default(),
        ],
        is_live: Some(true),
        do_spam_check: Some(false),
        send_emails: Some(true),
        populate_notifications: Some(false),
    };
    let _responses: Vec<models::SaveCommentsBulkResponse> = save_comments_bulk(&configuration, params).await?;
    Ok(())
}
[inline-code-end]