## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳：[`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_from_comment_response.rs)

## 範例

[inline-code-attrs-start title = 'get_ban_users_from_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_banned_users_from_comment() -> Result<GetBannedUsersFromCommentResponse, Error> {
    let params: GetBanUsersFromCommentParams = GetBanUsersFromCommentParams {
        comment_id: String::from("news/tech/acme-launch/comment-42"),
        sso: Some(String::from("acme-corp-sso-token-2026-06")),
    };
    let response: GetBannedUsersFromCommentResponse =
        get_ban_users_from_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---