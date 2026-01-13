## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |

## Respuesta

Devuelve: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_package_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tenant_package(configuration: &configuration::Configuration) -> Result<GetTenantPackage200Response, Error> {
    let params: GetTenantPackageParams = GetTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "premium-plan".to_string(),
        include_details: Some(true),
    };
    let package: GetTenantPackage200Response = get_tenant_package(configuration, params).await?;
    Ok(package)
}
[inline-code-end]

---