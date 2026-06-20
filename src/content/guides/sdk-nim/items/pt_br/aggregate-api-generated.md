Agrega documentos agrupando-os (se groupBy for fornecido) e aplicando múltiplas operações. Diferentes operações (por exemplo, sum, countDistinct, avg, etc.) são suportadas.

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| aggregationRequest | AggregationRequest | Não |  |
| parentTenantId | string | Não |  |
| includeStats | bool | Não |  |

## Resposta

Retorna: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de aggregate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---