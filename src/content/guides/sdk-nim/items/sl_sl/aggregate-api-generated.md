## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Ne |  |
| parentTenantId | string | Ne |  |
| includeStats | bool | Ne |  |

## Odgovor

Vrne: [`Option[AggregationResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregation_response.nim)

## Primer

[inline-code-attrs-start title = 'aggregate Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(),
  parentTenantId = "",
  includeStats = false
)
if response.isSome:
  let aggregation = response.get()
  echo $aggregation
[inline-code-end]

---