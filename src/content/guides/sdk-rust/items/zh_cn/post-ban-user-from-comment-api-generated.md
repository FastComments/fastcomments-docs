## 参数

| Name | Type | 必填 | 说明 |
|------|------|------|-------------|
| comment_id | String | 是 |  |
| ban_email | bool | 否 |  |
| ban_email_domain | bool | 否 |  |
| ban_ip | bool | 否 |  |
| delete_all_users_comments | bool | 否 |  |
| banned_until | String | 否 |  |
| is_shadow_ban | bool | 否 |  |
| update_id | String | 否 |  |
| ban_reason | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## 示例

[inline-code-attrs-start title = 'post_ban_user_from_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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