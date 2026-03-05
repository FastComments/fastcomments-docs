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
async fn pin_comment_example() -> Result<PinComment200Response, Error> {
    let params: PinCommentParams = PinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/2026/interesting-comment-987".to_string(),
        broadcast_id: "broadcast-nyc-1".to_string(),
        sso: Some("sso-token-xyz-789".to_string()),
    };
    let pinned: PinComment200Response = pin_comment(&configuration, params).await?;
    Ok(pinned)
}
[inline-code-end]
