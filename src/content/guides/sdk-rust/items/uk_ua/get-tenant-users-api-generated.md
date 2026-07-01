## Parameters

| Ім'я | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenant_id | String | Так |  |
| skip | f64 | Ні |  |

## Response

Повертає: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_tenant_users Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let _response = get_tenant_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]