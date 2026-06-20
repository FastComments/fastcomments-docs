## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config_response.nim)

## Örnek

[inline-code-attrs-start title = 'getQuestionConfig Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qst-456")
if response.isSome:
  let cfg = response.get()
  discard cfg
[inline-code-end]