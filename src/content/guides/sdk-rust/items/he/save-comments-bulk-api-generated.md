## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_comment_params | Vec<models::CreateCommentParams> | כן |  |
| is_live | bool | לא |  |
| do_spam_check | bool | לא |  |
| send_emails | bool | לא |  |
| populate_notifications | bool | לא |  |

## תגובה

מחזיר: `Vec<models::SaveCommentsBulkResponse>`

## דוגמה

[inline-code-attrs-start title = 'save_comments_bulk דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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