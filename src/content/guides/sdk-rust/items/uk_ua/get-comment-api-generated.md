## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |

## Відповідь

Повертає: [`ApiGetCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comment_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment() -> Result<(), Error> {
    let params = GetCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        include_deleted: Some(false),
    };

    let _response: ApiGetCommentResponse = get_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]