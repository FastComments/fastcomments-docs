Aggregere dokumenter ved at gruppere dem (hvis `groupBy` er angivet) og anvende flere operationer. Forskellige operationer (f.eks. `sum`, `countDistinct`, `avg` osv.) understøttes.

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Nej |  |
| options | AggregateOptions | Nej |  |

## Response

Returns: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Example

[inline-code-attrs-start title = 'aggregat Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]