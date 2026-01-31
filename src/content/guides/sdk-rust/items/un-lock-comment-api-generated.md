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
async fn run_unlock_comment() -> Result<(), Error> {
    let params: UnLockCommentParams = UnLockCommentParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        comment_id: "cmt-7890".to_owned(),
        broadcast_id: "news/article/2026/01/13".to_owned(),
        sso: Some("user-12345-token".to_owned()),
    };
    let response: LockComment200Response = un_lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
