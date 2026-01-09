Združuje dokumente z njihovo grupiranjem (če je podan groupBy) in izvajanjem več operacij. Podprte so različne operacije (npr. sum, countDistinct, avg itd.).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Da |  |
| parentTenantId | string | Ne |  |
| includeStats | boolean | Ne |  |

## Odgovor

Vrne: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---