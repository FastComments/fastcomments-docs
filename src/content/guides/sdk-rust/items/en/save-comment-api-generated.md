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
let create_comment_params: models::CreateCommentParams = models::CreateCommentParams {
    content: "I found this article very insightful and well-researched.".to_string(),
    url: "https://news.example.com/articles/2026/fast-comments".to_string(),
    author_name: Some("Jane Doe".to_string()),
    author_email: Some("jane.doe@example.com".to_string()),
    parent_id: None,
    tags: Some(vec!["technology".to_string(), "opinion".to_string()]),
};

let params: SaveCommentParams = SaveCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_comment_params,
    is_live: Some(true),
    do_spam_check: Some(true),
    send_emails: Some(false),
    populate_notifications: Some(true),
};

let response: SaveComment200Response = save_comment(&configuration, params).await?;
[inline-code-end]
