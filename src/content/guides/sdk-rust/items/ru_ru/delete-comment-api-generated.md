## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| context_user_id | String | Нет |  |
| is_live | bool | Нет |  |

## Ответ

Возвращает: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример delete_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_comment() -> Result<DeleteComment200Response, Error> {
    let params: DeleteCommentParams = DeleteCommentParams {
        tenant_id: "acme-newsroom".to_string(),
        id: "news/article-2026/comments/abc123".to_string(),
        context_user_id: Some("user-789".to_string()),
        is_live: Some(true),
    };
    let response: DeleteComment200Response = delete_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---