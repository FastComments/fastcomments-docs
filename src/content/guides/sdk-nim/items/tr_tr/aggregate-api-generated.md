Belgeleri gruplayarak (groupBy sağlanmışsa) ve birden çok işlem uygulayarak toplar.
Farklı işlemler (örn. sum, countDistinct, avg, vb.) desteklenir.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| aggregationRequest | AggregationRequest | Hayır |  |
| parentTenantId | string | Hayır |  |
| includeStats | bool | Hayır |  |

## Yanıt

Döndürür: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Örnek

[inline-code-attrs-start title = 'aggregate Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(groupBy = @["articleId"], metrics = @["commentCount"], filters = @[], limit = 0),
  parentTenantId = "",
  includeStats = false
)

if response.isSome:
  let agg = response.get()
  discard agg
[inline-code-end]

---