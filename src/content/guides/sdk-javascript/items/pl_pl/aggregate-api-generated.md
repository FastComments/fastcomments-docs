---
Agreguje dokumenty, grupując je (jeśli podano groupBy) i stosując wiele operacji. Obsługiwane są różne operacje (np. sum, countDistinct, avg itd.).

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| aggregationRequest | AggregationRequest | Tak |  |
| parentTenantId | string | Nie |  |
| includeStats | boolean | Nie |  |

## Odpowiedź

Zwraca: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---