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
async fn update_comment_text_example() -> Result<(), Error> {
    let params: SetCommentTextParams = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345/comment/67890".to_string(),
        broadcast_id: "news/daily/2026-01-13".to_string(),
        comment_text_update_request: CommentTextUpdateRequest {
            text: "Updated: Thanks for the detailed reporting — I respectfully disagree with point 3.".to_string(),
            ..Default::default()
        },
        edit_key: Some("edit-key-9f8a7b6c".to_string()),
        sso: Some("sso-token-user-42".to_string()),
    };

    let response: SetCommentText200Response = set_comment_text(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]
