## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_comment_params | models::CreateCommentParams | Yes |  |
| is_live | bool | No |  |
| do_spam_check | bool | No |  |
| send_emails | bool | No |  |
| populate_notifications | bool | No |  |

## Response

Returns: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'save_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
