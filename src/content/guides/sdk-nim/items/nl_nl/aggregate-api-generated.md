## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Nee |  |
| parentTenantId | string | Nee |  |
| includeStats | bool | Nee |  |

## Respons

Retourneert: [`Option[AggregationResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregation_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'aggregate Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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