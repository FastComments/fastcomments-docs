## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| id | String | Так |  |

## Відповідь

Повертає: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_tenant_user Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = GetTenantUserParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "user-42".into(),
    };
    let _response = get_tenant_user(&config, params).await?;
    Ok(())
}
[inline-code-end]