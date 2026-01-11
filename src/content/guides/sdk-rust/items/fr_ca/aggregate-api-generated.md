Aggrège des documents en les regroupant (si groupBy est fourni) et en appliquant plusieurs opérations. Différentes opérations (p. ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| aggregation_request | models::AggregationRequest | Oui |  |
| parent_tenant_id | String | Non |  |
| include_stats | bool | Non |  |

## Réponse

Renvoie: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---