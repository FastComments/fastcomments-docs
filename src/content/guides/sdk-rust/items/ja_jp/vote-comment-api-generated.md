## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| url_id | String | はい |  |
| broadcast_id | String | はい |  |
| vote_body_params | models::VoteBodyParams | はい |  |
| session_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

返却: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## 例

[inline-code-attrs-start title = 'vote_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: VoteCommentParams = VoteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        url_id: "news/politics/2026-election".to_string(),
        broadcast_id: "broadcast-nytimes-001".to_string(),
        vote_body_params: models::VoteBodyParams { ..Default::default() },
        session_id: Some("sess-9f8e7d".to_string()),
        sso: Some("user-42@example.com".to_string()),
    };
    let response: VoteComment200Response = vote_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---