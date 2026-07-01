Belgeleri gruplandırarak (eğer groupBy sağlanmışsa) ve birden fazla işlem uygulayarak toplar.  
Farklı işlemler (ör. sum, countDistinct, avg, vb.) desteklenir.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| aggregationRequest | AggregationRequest | Hayır |  |
| options | AggregateOptions | Hayır |  |

## Yanıt

Döndürür: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Örnek

[inline-code-attrs-start title = 'aggregate Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]