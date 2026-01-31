## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`LockComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/lock_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'lock_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<LockComment200Response, Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/2026/01/13/comment/98765".to_string(),
        broadcast_id: "live-sports-2026-01-13".to_string(),
        sso: Some("sso-token-0a1b2c3d".to_string()),
    };
    let response: LockComment200Response = lock_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
