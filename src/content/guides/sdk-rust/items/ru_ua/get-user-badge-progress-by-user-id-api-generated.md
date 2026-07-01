## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| user_id | String | Так |  |

## Відповідь

Повертає: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: "user-9876".to_string(),
    };
    let _response = get_user_badge_progress_by_user_id(&config, params).await?;
    Ok(())
}
[inline-code-end]