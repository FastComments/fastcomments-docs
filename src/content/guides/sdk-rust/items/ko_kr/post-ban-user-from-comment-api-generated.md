## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| ban_email | bool | 아니오 |  |
| ban_email_domain | bool | 아니오 |  |
| ban_ip | bool | 아니오 |  |
| delete_all_users_comments | bool | 아니오 |  |
| banned_until | String | 아니오 |  |
| is_shadow_ban | bool | 아니오 |  |
| update_id | String | 아니오 |  |
| ban_reason | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## 예제

[inline-code-attrs-start title = 'post_ban_user_from_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PostBanUserFromCommentParams = PostBanUserFromCommentParams {
    comment_id: "news-article-98765-comment-123".to_string(),
    ban_email: Some(true),
    ban_email_domain: Some(false),
    ban_ip: Some(true),
    delete_all_users_comments: Some(true),
    banned_until: Some("2026-12-31T23:59:59Z".to_string()),
    is_shadow_ban: Some(false),
    update_id: Some("moderator-42".to_string()),
    ban_reason: Some("Repeated spam and abusive language".to_string()),
    sso: Some("sso-user-token-8a7f".to_string()),
};
let ban_result: BanUserFromCommentResult = post_ban_user_from_comment(&configuration, params).await?;
[inline-code-end]

---