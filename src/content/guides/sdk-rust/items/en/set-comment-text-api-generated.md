## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Response

Returns: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

## Example

[inline-code-attrs-start title = 'set_comment_text Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_comment() -> Result<SetCommentText200Response, Error> {
    let params: SetCommentTextParams = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-20260110-78".to_string(),
        broadcast_id: "news/article/2026/interesting-story".to_string(),
        comment_text_update_request: models::CommentTextUpdateRequest {
            text: "Thanks for the update — here is a clarification with a link: https://example.com".to_string(),
        },
        edit_key: Some("edit-key-9f8a7".to_string()),
        sso: Some("sso-token-abcdef123456".to_string()),
    };
    let response: SetCommentText200Response = set_comment_text(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
