Agrega documentos agrupándolos (si se proporciona groupBy) y aplicando múltiples operaciones.
Se admiten diferentes operaciones (por ejemplo sum, countDistinct, avg, etc.).

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| aggregationRequest | AggregationRequest | Sí |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Respuesta

Devuelve: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---