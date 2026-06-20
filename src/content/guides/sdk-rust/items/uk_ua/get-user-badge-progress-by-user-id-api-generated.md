## Параметри

| Назва | Type | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| user_id | String | Так |  |

## Відповідь

Повертає: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_badge_progress_by_user_id'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let user_id_opt: Option<&str> = Some("user-7823");
    let params: GetUserBadgeProgressByUserIdParams = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: user_id_opt.unwrap().to_string(),
    };
    let response: ApiGetUserBadgeProgressResponse =
        get_user_badge_progress_by_user_id(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---