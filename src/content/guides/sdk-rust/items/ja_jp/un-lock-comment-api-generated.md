## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| broadcast_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

返却: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'un_lock_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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