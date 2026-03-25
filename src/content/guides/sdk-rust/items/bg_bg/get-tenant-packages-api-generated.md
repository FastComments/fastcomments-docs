## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| skip | f64 | Не |  |

## Отговор

Връща: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_200_response.rs)

## Пример

[inline-code-attrs-start title = 'get_tenant_packages Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetTenantPackagesParams = GetTenantPackagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let packages: GetTenantPackages200Response = get_tenant_packages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---