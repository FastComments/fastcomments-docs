## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| skip | f64 | No |  |

## Respuesta

Devuelve: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenant_packages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantPackagesParams = GetTenantPackagesParams {
        tenant_id: String::from("acme-corp-tenant"),
        skip: Some(10.0),
    };
    let response: GetTenantPackagesResponse = get_tenant_packages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---