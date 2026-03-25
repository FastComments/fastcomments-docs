---
## Parámetros

| Nombre | Type | Obligatorio | Descripción |
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
        tenant_id: "acme-corp-tenant".to_string(),
        meta: Some("news/article".to_string()),
        skip: Some(10.0),
    };
    let response: GetTenants200Response = get_tenants(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---