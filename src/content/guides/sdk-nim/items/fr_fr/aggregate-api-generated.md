Agrège des documents en les groupant (si groupBy est fourni) et en appliquant plusieurs opérations.
Différentes opérations (p. ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| aggregationRequest | AggregationRequest | Non |  |
| parentTenantId | string | Non |  |
| includeStats | bool | Non |  |

## Réponse

Renvoie : [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'agrégation'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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