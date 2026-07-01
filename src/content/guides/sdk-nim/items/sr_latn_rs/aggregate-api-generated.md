Aggregira dokumente grupisanjem (ako je groupBy prosleđen) i primenom više operacija. Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Ne |  |
| options | AggregateOptions | Ne |  |

## Response

Returns: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Primer

[inline-code-attrs-start title = 'aggregate Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]