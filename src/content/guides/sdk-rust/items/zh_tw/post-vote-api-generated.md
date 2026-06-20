## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| direction | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳：[`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## 範例

[inline-code-attrs-start title = 'post_vote 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn submit_vote() -> Result<VoteResponse, Error> {
    let params: PostVoteParams = PostVoteParams {
        comment_id: String::from("news/article-1234/comment-5678"),
        direction: Some(String::from("up")),
        sso: Some(String::from("acme-corp-sso-token-abc123")),
    };
    let vote_response: VoteResponse = post_vote(&configuration, params).await?;
    Ok(vote_response)
}
[inline-code-end]

---