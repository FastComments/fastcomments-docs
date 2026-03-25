## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| context_user_id | String | Ні |  |
| is_live | bool | Ні |  |

## Відповідь

Повертає: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'delete_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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