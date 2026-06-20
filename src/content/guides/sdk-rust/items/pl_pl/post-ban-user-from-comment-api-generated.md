## Parametry

| Name | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| comment_id | String | Tak |  |
| ban_email | bool | Nie |  |
| ban_email_domain | bool | Nie |  |
| ban_ip | bool | Nie |  |
| delete_all_users_comments | bool | Nie |  |
| banned_until | String | Nie |  |
| is_shadow_ban | bool | Nie |  |
| update_id | String | Nie |  |
| ban_reason | String | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## Przykład

[inline-code-attrs-start title = 'post_ban_user_from_comment Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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