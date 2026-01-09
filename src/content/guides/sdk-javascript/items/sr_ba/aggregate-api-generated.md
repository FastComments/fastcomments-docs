---
Agregira dokumente grupisanjem (ako je groupBy naveden) i primjenom više operacija. Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Da |  |
| parentTenantId | string | Ne |  |
| includeStats | boolean | Ne |  |

## Odgovor

Vraća: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---