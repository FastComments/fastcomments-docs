## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenant_id | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_count_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_counts'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetCountsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("news/article".to_string()),
    };
    let _response = get_counts(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---