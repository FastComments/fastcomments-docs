## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| broadcast_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`LockComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/lock_comment_200_response.rs)

## 例

[inline-code-attrs-start title = 'lock_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: String::from("acme-news-tenant"),
        comment_id: String::from("cmt-20260325-789"),
        broadcast_id: String::from("live/politics-debate-2026-03-25"),
        sso: Some(String::from("sso-user-0a1b2c3d4e")),
    };
    let response: LockComment200Response = lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---