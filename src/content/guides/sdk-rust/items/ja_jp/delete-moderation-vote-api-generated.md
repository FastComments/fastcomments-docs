## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| comment_id | String | はい |  |
| vote_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## 例

[inline-code-attrs-start title = 'delete_moderation_vote の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteModerationVoteParams = DeleteModerationVoteParams {
        comment_id: "news/article-2026-06-19-12345".to_string(),
        vote_id: "vote-9a7c3b1d".to_string(),
        sso: Some("user-9876@acme-corp".to_string()),
    };
    let response: VoteDeleteResponse = delete_moderation_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---