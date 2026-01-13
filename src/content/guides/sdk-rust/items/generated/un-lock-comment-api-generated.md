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

[inline-code-attrs-start title = 'un_lock_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unlock() -> Result<(), Error> {
    let params: UnLockCommentParams = UnLockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-9f2b4a".to_string(),
        broadcast_id: "news/world/2026/01/12/live-roundup".to_string(),
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: LockComment200Response = un_lock_comment(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]
