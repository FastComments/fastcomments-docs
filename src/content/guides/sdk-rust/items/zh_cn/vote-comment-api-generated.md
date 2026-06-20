## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| url_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| vote_body_params | models::VoteBodyParams | 是 |  |
| session_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## 示例

[inline-code-attrs-start title = 'vote_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: VoteCommentParams = VoteCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt_8392a1".to_string(),
    url_id: "news/article-2026-06-19-rust-release".to_string(),
    broadcast_id: "broadcast_2026_06".to_string(),
    vote_body_params: models::VoteBodyParams { value: 1 },
    session_id: Some("sess_4f9b2c".to_string()),
    sso: Some("sso_token_abcd1234".to_string()),
};
let response: VoteResponse = vote_comment(&configuration, params).await?;
[inline-code-end]

---