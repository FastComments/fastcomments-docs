## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Одговор

Враћа: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_200_response.rs)

## Пример

[inline-code-attrs-start title = 'get_tenant Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenant() -> Result<(), Error> {
    let params: GetTenantParams = GetTenantParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article"),
    };
    let include_metadata: Option<bool> = Some(true);
    let tenant_response: GetTenant200Response = get_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---