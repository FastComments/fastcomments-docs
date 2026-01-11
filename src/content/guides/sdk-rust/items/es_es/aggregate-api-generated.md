Realiza agregaciones de documentos agrupándolos (si se proporciona groupBy) y aplicando múltiples operaciones.
Se admiten diferentes operaciones (p. ej. sum, countDistinct, avg, etc.).

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| aggregation_request | models::AggregationRequest | Sí |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Respuesta

Devuelve: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---