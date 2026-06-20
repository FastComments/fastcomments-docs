## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| skip | f64 | Ні |  |

## Відповідь

Повертає: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_tenant_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetTenantUsersParams = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetTenantUsersResponse = get_tenant_users(&configuration, params).await?;
    let _users: GetTenantUsersResponse = response;
    Ok(())
}
[inline-code-end]

---