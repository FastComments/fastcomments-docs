---
## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| edit_key | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comment_text'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn run() -> Result<GetCommentText200Response, Error> {
    let params: GetCommentTextParams = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2026-03-25-98765".to_string(),
        edit_key: Some("edit_4f3d2b9a".to_string()),
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
    };
    let comment: GetCommentText200Response = get_comment_text(&configuration, params).await?;
    Ok(comment)
}
[inline-code-end]

---