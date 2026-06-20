Agregira dokumente grupišući ih (ako je groupBy naveden) i primenjujući više operacija.
Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Ne |  |
| parentTenantId | string | Ne |  |
| includeStats | bool | Ne |  |

## Odgovor

Vraća: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer za aggregate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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