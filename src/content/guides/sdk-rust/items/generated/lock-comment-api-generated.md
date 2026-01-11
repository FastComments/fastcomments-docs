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
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-98765".to_string(),
        broadcast_id: "news/live/2025/11/21/article-123".to_string(),
        sso: Some("sso-token-7f3a2b".to_string()),
    };
    let response: LockComment200Response = lock_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]
