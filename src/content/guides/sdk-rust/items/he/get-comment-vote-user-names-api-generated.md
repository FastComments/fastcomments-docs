## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| comment_id | String | כן |  |
| dir | i32 | כן |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comment_vote_user_names'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_vote_user_names() -> Result<GetCommentVoteUserNames200Response, Error> {
    let params: GetCommentVoteUserNamesParams = GetCommentVoteUserNamesParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("news/article-12345/comment-6789"),
        dir: 1,
        sso: Some(String::from("sso-token-01a2b3")),
    };
    let response: GetCommentVoteUserNames200Response =
        get_comment_vote_user_names(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---