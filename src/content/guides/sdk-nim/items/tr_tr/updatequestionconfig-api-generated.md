## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Hayır |  |

## Yanıt

Döndürür: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Örnek

[inline-code-attrs-start title = 'updateQuestionConfig Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionConfig(tenantId = "my-tenant-123", id = "question-config-456", updateQuestionConfigBody = default(UpdateQuestionConfigBody))
if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---