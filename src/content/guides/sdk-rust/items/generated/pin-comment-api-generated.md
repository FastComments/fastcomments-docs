## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'pin_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_pin() -> Result<PinComment200Response, Error> {
    let params: PinCommentParams = PinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news-article-comment-4721".to_string(),
        broadcast_id: "news/article/2025-11-21-live".to_string(),
        sso: Some("user-42-sso-token".to_string()),
    };
    let response: PinComment200Response = pin_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
