## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| adjust_comment_votes_params | models::AdjustCommentVotesParams | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/adjust_votes_response.rs)

## 示例

[inline-code-attrs-start title = 'post_adjust_comment_votes 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PostAdjustCommentVotesParams = PostAdjustCommentVotesParams {
    comment_id: String::from("cmt-news-2026-0042"),
    adjust_comment_votes_params: models::AdjustCommentVotesParams {
        delta: 1,
        reason: Some(String::from("Added supporting source")),
    },
    sso: Some(String::from("sso-acme-corp-tenant-xyz123")),
};
let response: AdjustVotesResponse = post_adjust_comment_votes(&configuration, params).await?;
[inline-code-end]

---