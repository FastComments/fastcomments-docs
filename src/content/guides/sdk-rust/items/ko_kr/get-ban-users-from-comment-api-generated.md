---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_from_comment_response.rs)

## 예제

[inline-code-attrs-start title = 'get_ban_users_from_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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