## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| comment_id | String | Yes |  |
| approved | bool | No |  |
| sso | String | No |  |

## Ответ

Возвращает: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## Пример

[inline-code-attrs-start title = 'post_set_comment_approval_status Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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