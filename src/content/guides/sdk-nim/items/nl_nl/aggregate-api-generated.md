Voert aggregatie uit op documenten door ze te groeperen (als groupBy is opgegeven) en meerdere bewerkingen toe te passen.
Verschillende bewerkingen (bijv. sum, countDistinct, avg, enz.) worden ondersteund.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Nee |  |
| parentTenantId | string | Nee |  |
| includeStats | bool | Nee |  |

## Respons

Geeft terug: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'aggregate Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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