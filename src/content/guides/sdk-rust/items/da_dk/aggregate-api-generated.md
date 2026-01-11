---
Aggregerer dokumenter ved at gruppere dem (hvis groupBy er angivet) og anvende flere operationer. Forskellige operationer (f.eks. sum, countDistinct, avg osv.) understøttes.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| aggregation_request | models::AggregationRequest | Ja |  |
| parent_tenant_id | String | Nej |  |
| include_stats | bool | Nej |  |

## Svar

Returnerer: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---