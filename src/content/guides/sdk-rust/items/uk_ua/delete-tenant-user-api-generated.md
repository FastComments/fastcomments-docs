## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| delete_comments | String | Ні |  |
| comment_delete_mode | String | Ні |  |

## Відповідь

Повертає: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Приклад

[inline-code-attrs-start title = 'delete_tenant_user Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteTenantUserParams {
        tenant_id: "acme-corp".into(),
        id: "user-123".into(),
        delete_comments: Some("true".into()),
        comment_delete_mode: Some("hard".into()),
    };
    delete_tenant_user(&config, params).await?;
    Ok(())
}
[inline-code-end]

---