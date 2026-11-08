## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| create_comment_params | models::CreateCommentParams | हाँ |  |
| is_live | bool | नहीं |  |
| do_spam_check | bool | नहीं |  |
| send_emails | bool | नहीं |  |
| populate_notifications | bool | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ApiSaveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_save_comment_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'save_comment उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = SaveCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: models::CreateCommentParams {
            body: "Great insights on the latest tech trends.".to_string(),
            user_id: "user-789".to_string(),
            ..Default::default()
        },
        is_live: Some(true),
        do_spam_check: Some(true),
        send_emails: Some(false),
        populate_notifications: Some(true),
    };
    let _response = save_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]