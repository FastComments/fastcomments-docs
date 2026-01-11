---
Agreguje dokumenty przez grupowanie ich (jeśli podano groupBy) i stosowanie wielu operacji.
Obsługiwane są różne operacje (np. sum, countDistinct, avg itd.).

## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| aggregation_request | models::AggregationRequest | Tak |  |
| parent_tenant_id | String | Nie |  |
| include_stats | bool | Nie |  |

## Odpowiedź

Zwraca: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---