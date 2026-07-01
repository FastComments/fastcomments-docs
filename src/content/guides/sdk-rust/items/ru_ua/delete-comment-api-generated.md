## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| context_user_id | String | Ні |  |
| is_live | bool | Ні |  |

## Відповідь

Повертає: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## Приклад

[inline-code-attrs-start title = 'delete_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        context_user_id: Some("user-6789".to_string()),
        is_live: Some(true),
    };
    let _result = delete_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]