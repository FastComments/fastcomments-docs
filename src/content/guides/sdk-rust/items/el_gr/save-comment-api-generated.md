## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| create_comment_params | models::CreateCommentParams | Ναι |  |
| is_live | bool | Όχι |  |
| do_spam_check | bool | Όχι |  |
| send_emails | bool | Όχι |  |
| populate_notifications | bool | Όχι |  |

## Απάντηση

Επιστρέφει: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'save_comment Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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