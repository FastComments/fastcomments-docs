---
Agreguje dokumenty poprzez grupowanie ich (jeśli podano groupBy) i stosowanie wielu operacji.
Obsługiwane są różne operacje (np. sum, countDistinct, avg, itp.).

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | No |  |
| options | AggregateOptions | No |  |

## Odpowiedź

Zwraca: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład agregacji'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]

---