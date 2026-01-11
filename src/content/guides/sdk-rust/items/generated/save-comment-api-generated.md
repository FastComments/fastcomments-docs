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
async fn run() -> Result<SaveComment200Response, Error> {
    let create_params: models::CreateCommentParams = models::CreateCommentParams {
        thread_id: "news/article/2025/11/21".to_string(),
        content: "Great article! The reporting on the budget was thorough and well-sourced.".to_string(),
        author_name: Some("Jane Doe".to_string()),
        author_email: Some("jane.doe@acme.com".to_string()),
    };
    let params: SaveCommentParams = SaveCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_comment_params: create_params,
        is_live: Some(true),
        do_spam_check: Some(true),
        send_emails: Some(false),
        populate_notifications: Some(true),
    };
    let response: SaveComment200Response = save_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
