## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| adjust_comment_votes_params | models::AdjustCommentVotesParams | 예 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/adjust_votes_response.rs)

## 예제

[inline-code-attrs-start title = 'post_adjust_comment_votes 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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