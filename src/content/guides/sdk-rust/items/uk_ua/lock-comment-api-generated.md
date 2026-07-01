## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| broadcast_id | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Приклад

[inline-code-attrs-start title = 'lock_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-9876".to_string(),
        broadcast_id: "news/article".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _resp = lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]