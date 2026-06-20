Agreguje dokumenty grupując je (jeśli podano groupBy) i stosując wiele operacji. Obsługiwane są różne operacje (np. sum, countDistinct, avg itd.).

## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| aggregationRequest | AggregationRequest | Nie |  |
| parentTenantId | string | Nie |  |
| includeStats | bool | Nie |  |

## Odpowiedź

Zwraca: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład agregacji'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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