## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| meta | String | No |  |
| skip | f64 | No |  |

## Respuesta

Devuelve: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenants'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantsParams = GetTenantsParams {
        tenant_id: String::from("acme-corp-tenant"),
        meta: Some(String::from("include=domains,settings")),
        skip: Some(10.0),
    };
    let response: GetTenants200Response = get_tenants(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---