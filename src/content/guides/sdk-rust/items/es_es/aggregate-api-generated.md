---
Agrupa documentos agrupándolos (si se proporciona `groupBy`) y aplicando múltiples operaciones.  
Se admiten diferentes operaciones (p. ej., `sum`, `countDistinct`, `avg`, etc.).

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenant_id | String | Sí |  |
| aggregation_request | models::AggregationRequest | Sí |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Respuesta

Devuelve: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de agregación'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let aggregation_request = models::AggregationRequest::default();
    let params = AggregateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        aggregation_request,
        parent_tenant_id: Some("global-tenant".to_string()),
        include_stats: Some(true),
    };
    let _response = aggregate(&config, params).await?;
    Ok(())
}
[inline-code-end]

---