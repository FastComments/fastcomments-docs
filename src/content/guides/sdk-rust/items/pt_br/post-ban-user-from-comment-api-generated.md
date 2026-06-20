## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| comment_id | String | Sim |  |
| ban_email | bool | Não |  |
| ban_email_domain | bool | Não |  |
| ban_ip | bool | Não |  |
| delete_all_users_comments | bool | Não |  |
| banned_until | String | Não |  |
| is_shadow_ban | bool | Não |  |
| update_id | String | Não |  |
| ban_reason | String | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de post_ban_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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