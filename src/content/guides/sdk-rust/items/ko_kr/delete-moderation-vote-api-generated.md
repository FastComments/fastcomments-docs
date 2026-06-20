## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| vote_id | String | 예 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_moderation_vote 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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