Agregira dokumente grupiranjem (ako je groupBy provided) i primjenom više operacija. Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| aggregation_request | models::AggregationRequest | Da |  |
| parent_tenant_id | String | Ne |  |
| include_stats | bool | Ne |  |

## Odgovor

Vraća: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---