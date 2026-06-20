## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| ban_email | bool | Nein |  |
| ban_email_domain | bool | Nein |  |
| ban_ip | bool | Nein |  |
| delete_all_users_comments | bool | Nein |  |
| banned_until | String | Nein |  |
| is_shadow_ban | bool | Nein |  |
| update_id | String | Nein |  |
| ban_reason | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## Beispiel

[inline-code-attrs-start title = 'post_ban_user_from_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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