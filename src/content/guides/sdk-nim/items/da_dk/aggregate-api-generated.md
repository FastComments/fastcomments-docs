Aggregerer dokumenter ved at gruppere dem (hvis groupBy er angivet) og anvende flere operationer. Forskellige operationer (f.eks. sum, countDistinct, avg osv.) understøttes.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Nej |  |
| parentTenantId | string | Nej |  |
| includeStats | bool | Nej |  |

## Respons

Returnerer: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Eksempel

[inline-code-attrs-start title = 'aggregate Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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