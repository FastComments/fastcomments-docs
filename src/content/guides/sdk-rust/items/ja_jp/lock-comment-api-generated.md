## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| sso | String | No |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'lock_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-9876".to_string(),
        broadcast_id: "news/article".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _resp = lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]