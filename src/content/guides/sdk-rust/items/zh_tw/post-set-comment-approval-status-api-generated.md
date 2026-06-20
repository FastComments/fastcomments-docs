## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| approved | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳：[`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## 範例

[inline-code-attrs-start title = 'post_set_comment_approval_status 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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