Agrège les documents en les regroupant (si groupBy est fourni) et en appliquant plusieurs opérations.
Différentes opérations (p. ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| aggregationRequest | AggregationRequest | Oui |  |
| parentTenantId | string | Non |  |
| includeStats | boolean | Non |  |

## Réponse

Renvoie : [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---