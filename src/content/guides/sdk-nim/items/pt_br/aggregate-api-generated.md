Agrega documentos agrupando‑os (se *groupBy* for fornecido) e aplicando várias operações. Diferentes operações (por exemplo, sum, countDistinct, avg, etc.) são suportadas.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| aggregationRequest | AggregationRequest | Não |  |
| options | AggregateOptions | Não |  |

## Resposta

Retorna: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de agregação'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]