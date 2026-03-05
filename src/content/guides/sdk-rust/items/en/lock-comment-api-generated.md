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
async fn lock_comment_example() -> Result<(), Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/98765#comment-42".to_string(),
        broadcast_id: "live-coverage-98765".to_string(),
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.payload".to_string()),
    };

    let response: LockComment200Response = lock_comment(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]
