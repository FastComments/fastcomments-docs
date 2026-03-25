## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Одговор

Враћа: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_package_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_tenant_package() -> Result<GetTenantPackage200Response, Error> {
    let params: GetTenantPackageParams = GetTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "package-basic-2026".to_string(),
    };
    let include_metadata: Option<bool> = Some(true);
    let package: GetTenantPackage200Response = get_tenant_package(&configuration, params).await?;
    Ok(package)
}
[inline-code-end]

---