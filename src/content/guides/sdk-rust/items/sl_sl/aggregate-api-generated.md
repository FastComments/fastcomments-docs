Združuje dokumente z grupiranjem (če je podano groupBy) in z izvajanjem več operacij. Podprte so različne operacije (npr. sum, countDistinct, avg itd.).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| aggregation_request | models::AggregationRequest | Da |  |
| parent_tenant_id | String | Ne |  |
| include_stats | bool | Ne |  |

## Odgovor

Vrne: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---