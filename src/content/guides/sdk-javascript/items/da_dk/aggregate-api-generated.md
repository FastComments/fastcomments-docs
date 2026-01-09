Aggregerer dokumenter ved at gruppere dem (hvis groupBy er angivet) og anvende flere operationer.
Forskellige operationer (f.eks. sum, countDistinct, avg osv.) understøttes.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Ja |  |
| parentTenantId | string | Nej |  |
| includeStats | boolean | Nej |  |

## Svar

Returnerer: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---