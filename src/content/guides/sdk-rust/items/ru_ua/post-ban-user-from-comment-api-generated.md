## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| comment_id | String | Да |  |
| ban_email | bool | Нет |  |
| ban_email_domain | bool | Нет |  |
| ban_ip | bool | Нет |  |
| delete_all_users_comments | bool | Нет |  |
| banned_until | String | Нет |  |
| is_shadow_ban | bool | Нет |  |
| update_id | String | Нет |  |
| ban_reason | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## Пример

[inline-code-attrs-start title = 'Пример post_ban_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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