## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| from_name | String | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад send_invite'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: SendInviteParams = SendInviteParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "articles/2026/01/ai-news-12345".to_string(),
    from_name: "Acme Newsroom".to_string(),
    reply_to: Some("editorial@acme.com".to_string()),
    message: Some("You have been invited to moderate comments on this article.".to_string()),
};

let invite_response: FlagCommentPublic200Response = send_invite(&configuration, params).await?;
[inline-code-end]

---