## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| dir | i32 | 是 |  |
| sso | String | 否 |  |

## 回應

回傳：[`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_comment_vote_user_names 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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