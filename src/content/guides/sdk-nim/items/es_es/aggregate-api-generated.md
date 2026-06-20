Agrega documentos agrupándolos (si se proporciona groupBy) y aplicando múltiples operaciones.
Se admiten distintas operaciones (p. ej. sum, countDistinct, avg, etc.).

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| aggregationRequest | AggregationRequest | No |  |
| parentTenantId | string | No |  |
| includeStats | bool | No |  |

## Respuesta

Devuelve: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de aggregate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(groupBy = @["articleId"], metrics = @["commentCount"], filters = @[], limit = 0),
  parentTenantId = "",
  includeStats = false
)

if response.isSome:
  let agg = response.get()
  discard agg
[inline-code-end]