## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| user_id | String | いいえ |  |
| anon_user_id | String | いいえ |  |

## レスポンス

戻り値: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_200_response.rs)

## 例

[inline-code-attrs-start title = 'flag_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagComment200Response, Error> {
    let params: FlagCommentParams = FlagCommentParams {
        tenant_id: "acme-news-tenant".to_string(),
        id: "comment-20260325-842".to_string(),
        user_id: Some("user-7b2f3d".to_string()),
        anon_user_id: Some("anon-1a2b3c".to_string()),
    };
    let resp: FlagComment200Response = flag_comment(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---