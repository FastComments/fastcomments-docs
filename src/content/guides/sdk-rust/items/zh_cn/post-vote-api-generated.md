## 参数

| 名称 | 类型 | 是否必填 | 说明 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| direction | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## 示例

[inline-code-attrs-start title = 'post_vote 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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