---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| edit_key | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`PublicApiGetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_get_comment_text_response.rs)

## Пример

[inline-code-attrs-start title = 'get_comment_text Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment_text() -> Result<PublicApiGetCommentTextResponse, Error> {
    let params = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2026-06-19#cmt-8421".to_string(),
        edit_key: Some("editkey-73a1b2c".to_string()),
        sso: Some("sso.jwt.token.eyJhbGci".to_string()),
    };
    let response: PublicApiGetCommentTextResponse = get_comment_text(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---