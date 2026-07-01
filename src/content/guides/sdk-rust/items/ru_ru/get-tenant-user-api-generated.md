## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Ответ

Возвращает: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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