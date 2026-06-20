## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| broadcast_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'un_lock_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UnLockCommentParams = UnLockCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("news/article-123#comment-4829"),
        broadcast_id: String::from("broadcast-2025-06-19"),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")),
    };
    let response: ApiEmptyResponse = un_lock_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---