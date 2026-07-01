Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.  
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|-------|-----|----------|------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Ne |  |
| options | AggregateOptions | Ne |  |

## Odgovor

Vraća: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Primjer

[inline-code-attrs-start title = 'aggregate Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]

---