## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenant_id | String | Так |  |
| id | String | Так |  |

## Відповідь

Повертає: [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn remove_badge(config: &configuration::Configuration) -> Result<(), Error> {
    let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-abcde".to_string(),
    };
    let _ = delete_user_badge(config, params).await?;
    Ok(())
}
[inline-code-end]