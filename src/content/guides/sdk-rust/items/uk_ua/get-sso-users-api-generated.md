## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| skip | i32 | Ні |  |

## Відповідь

Повертає: [`GetSsoUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_sso_users Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetSsoUsersResponse, Error> {
    let params: GetSsoUsersParams = GetSsoUsersParams {
        tenant_id: String::from("acme-corp-tenant"),
        skip: Some(10),
    };
    let sso_users: GetSsoUsersResponse = get_sso_users(&configuration, params).await?;
    Ok(sso_users)
}
[inline-code-end]

---