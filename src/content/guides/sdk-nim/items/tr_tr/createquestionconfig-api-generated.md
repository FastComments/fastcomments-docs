## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Hayır |  |

## Yanıt

Döndürür: [`Option[CreateQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config200response.nim)

## Örnek

[inline-code-attrs-start title = 'createQuestionConfig Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = CreateQuestionConfigBody(
  key = "recommendation",
  question = "Would you recommend this article to a friend?",
  required = false,
  inputType = "radio",
  options = @["Yes", "No", "Maybe"]
)

let (response, httpResponse) = client.createQuestionConfig(tenantId = "my-tenant-123", createQuestionConfigBody = body)

if response.isSome:
  let config = response.get()
  discard config
[inline-code-end]

---