## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| url_id | String | Так |  |
| broadcast_id | String | Так |  |
| vote_body_params | models::VoteBodyParams | Так |  |
| session_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Приклад

[inline-code-attrs-start title = 'vote_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: VoteCommentParams = VoteCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt_8392a1".to_string(),
    url_id: "news/article-2026-06-19-rust-release".to_string(),
    broadcast_id: "broadcast_2026_06".to_string(),
    vote_body_params: models::VoteBodyParams { value: 1 },
    session_id: Some("sess_4f9b2c".to_string()),
    sso: Some("sso_token_abcd1234".to_string()),
};
let response: VoteResponse = vote_comment(&configuration, params).await?;
[inline-code-end]

---