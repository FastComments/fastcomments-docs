Agrège les documents en les regroupant (si groupBy est fourni) et en appliquant plusieurs opérations.  
Différentes opérations (par ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| aggregationRequest | AggregationRequest | Non |  |
| options | AggregateOptions | Non |  |

## Réponse

Renvoie : [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'agrégation'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]