---
Agregira dokumente grupišući ih (ako je groupBy prosleđen) i primenjujući više operacija.
Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Da |  |
| parentTenantId | string | Ne |  |
| includeStats | boolean | Ne |  |

## Odgovor

Vraća: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---