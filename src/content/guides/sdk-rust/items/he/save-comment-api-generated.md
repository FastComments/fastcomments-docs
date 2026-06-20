## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_comment_params | models::CreateCommentParams | כן |  |
| is_live | bool | לא |  |
| do_spam_check | bool | לא |  |
| send_emails | bool | לא |  |
| populate_notifications | bool | לא |  |

## תשובה

מחזיר: [`ApiSaveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_save_comment_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-save_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<ApiSaveCommentResponse, Error> {
    let create_comment_params: models::CreateCommentParams = models::CreateCommentParams {
        content: "Great in-depth coverage of the Rust 2026 release!".to_string(),
        author_id: Some("user-4821".to_string()),
        author_name: Some("Jamie Morgan".to_string()),
        permalink: Some("https://news.example.com/articles/2026/rust-release".to_string()),
        parent_id: None,
        metadata: None,
    };
    let params: SaveCommentParams = SaveCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params,
        is_live: Some(true),
        do_spam_check: Some(true),
        send_emails: Some(false),
        populate_notifications: Some(true),
    };
    let response: ApiSaveCommentResponse = save_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---