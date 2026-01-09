Belgeleri, (groupBy sağlanmışsa) gruplayarak ve birden çok işlem uygulayarak toplar.
Farklı işlemler (ör. sum, countDistinct, avg vb.) desteklenir.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| aggregationRequest | AggregationRequest | Evet |  |
| parentTenantId | string | Hayır |  |
| includeStats | boolean | Hayır |  |

## Yanıt

Döndürür: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---