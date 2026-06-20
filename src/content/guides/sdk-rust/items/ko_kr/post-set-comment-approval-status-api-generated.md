## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| approved | bool | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## 예제

[inline-code-attrs-start title = 'post_set_comment_approval_status 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: PostSetCommentApprovalStatusParams = PostSetCommentApprovalStatusParams {
        comment_id: String::from("news/article/2026-06-19/post-42/comment-128"),
        approved: Some(true),
        sso: Some(String::from("sso:user:acme:eyJhbGciOiJIUzI1Ni")),
    };
    let response: SetCommentApprovedResponse = post_set_comment_approval_status(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---