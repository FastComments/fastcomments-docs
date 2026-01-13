---
Aggregeert documenten door ze te groeperen (als groupBy is opgegeven) en meerdere bewerkingen toe te passen.
Verschillende bewerkingen (bijv. sum, countDistinct, avg, enz.) worden ondersteund.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| aggregation_request | models::AggregationRequest | Ja |  |
| parent_tenant_id | String | Nee |  |
| include_stats | bool | Nee |  |

## Response

Retourneert: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---