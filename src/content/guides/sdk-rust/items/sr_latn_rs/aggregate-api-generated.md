Agregira dokumente grupisanjem (ako je groupBy naveden) i primenom više operacija. Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| aggregation_request | models::AggregationRequest | Da |  |
| parent_tenant_id | String | Ne |  |
| include_stats | bool | Ne |  |

## Odgovor

Vraća: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---