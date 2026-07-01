Agregira dokumente grupišući ih (ako je groupBy naveden) i primenjujući više operacija.  
Različite operacije (npr. sum, countDistinct, avg, itd.) su podržane.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | No |  |
| options | AggregateOptions | No |  |

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