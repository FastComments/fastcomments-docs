## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| broadcast_id | String | 예 |  |
| sso | String | 아니오 |  |

## Response

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Example

[inline-code-attrs-start title = 'un_lock_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = UnLockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-456".to_string(),
        broadcast_id: "news/article-123".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response = un_lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]