## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Response

Returns: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_comment_text Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment_text() -> Result<(), Error> {
    let params: GetCommentTextParams = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2025-11-21-12345".to_string(),
        edit_key: Some("edit_9f8b7c".to_string()),
        sso: Some("sso_token_eyJhbGci".to_string()),
    };
    let response: GetCommentText200Response = get_comment_text(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]
