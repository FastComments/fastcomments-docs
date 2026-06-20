## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |

## Respuesta

Devuelve: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_package_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenant_package() -> Result<GetTenantPackageResponse, Error> {
    let params: GetTenantPackageParams = GetTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "pkg-premium-001".to_string(),
        include_related: Some(true),
    };
    let response: GetTenantPackageResponse = get_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---