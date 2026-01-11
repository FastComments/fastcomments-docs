Aggrega documenti raggruppandoli (se viene fornito groupBy) e applicando più operazioni.
Sono supportate diverse operazioni (es. sum, countDistinct, avg, ecc.).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| aggregation_request | models::AggregationRequest | Sì |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Risposta

Restituisce: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---