## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
| comment_id | String | はい |  |
| adjust_comment_votes_params | models::AdjustCommentVotesParams | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/adjust_votes_response.rs)

## 例

[inline-code-attrs-start title = 'post_adjust_comment_votes の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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