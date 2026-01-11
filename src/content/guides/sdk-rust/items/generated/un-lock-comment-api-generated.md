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
async fn example_un_lock_comment() -> Result<LockComment200Response, Error> {
    let params: UnLockCommentParams = UnLockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-9a8b7".to_string(),
        broadcast_id: "live/news/2025-11-21-08:00".to_string(),
        sso: Some("sso-token-4f2b".to_string()),
    };
    let response: LockComment200Response = un_lock_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
