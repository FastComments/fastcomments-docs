Belgeleri gruplayarak (groupBy sağlanmışsa) ve birden fazla işlem uygulayarak toplar. Farklı işlemler (ör. sum, countDistinct, avg vb.) desteklenir.

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| aggregation_request | models::AggregationRequest | Evet |  |
| parent_tenant_id | String | Hayır |  |
| include_stats | bool | Hayır |  |

## Yanıt

Döndürür: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---