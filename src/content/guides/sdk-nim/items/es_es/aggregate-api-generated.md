Aggrega documentos agrupándolos (si se proporciona `groupBy`) y aplicando múltiples operaciones.  
Se admiten diferentes operaciones (p. ej., sum, countDistinct, avg, etc.).

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| aggregationRequest | AggregationRequest | No |  |
| options | AggregateOptions | No |  |

## Respuesta

Devuelve: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de agregación'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]