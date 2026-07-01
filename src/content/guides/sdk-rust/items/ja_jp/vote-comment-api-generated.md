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

返却: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## 例

[inline-code-attrs-start title = 'vote_comment 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config = configuration::Configuration::default();

    let vote_body = models::VoteBodyParams {
        vote_type: "upvote".to_string(),
        weight: 1,
    };

    let params = VoteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        url_id: "news/article".to_string(),
        broadcast_id: "broadcast-67890".to_string(),
        vote_body_params: vote_body,
        session_id: Some("session-abcde".to_string()),
        sso: None,
    };

    let _response = vote_comment(&config, params).await?;
    Ok(())
}
[inline-code-end]

---