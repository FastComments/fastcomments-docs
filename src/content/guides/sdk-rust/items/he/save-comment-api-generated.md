## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_comment_params | models::CreateCommentParams | כן |  |
| is_live | bool | לא |  |
| do_spam_check | bool | לא |  |
| send_emails | bool | לא |  |
| populate_notifications | bool | לא |  |

## תשובה

מחזיר: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-save_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let create_comment: models::CreateCommentParams = models::CreateCommentParams {
    thread_key: "news/article/2026/03/25/budget-reform".to_string(),
    body: "Great breakdown of the proposed changes — very informative and balanced.".to_string(),
    user_name: Some("Jane Doe".to_string()),
    user_email: Some("jane.doe@acme-corp.com".to_string()),
    user_id: Some("user-9876".to_string()),
    parent_id: None,
};

let save_params: SaveCommentParams = SaveCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_comment_params: create_comment,
    is_live: Some(true),
    do_spam_check: Some(true),
    send_emails: Some(false),
    populate_notifications: Some(true),
};

let saved: SaveComment200Response = save_comment(&configuration, save_params).await?;
[inline-code-end]

---