## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| url_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| vote_body_params | models::VoteBodyParams | 是 |  |
| session_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## 示例

[inline-code-attrs-start title = 'vote_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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