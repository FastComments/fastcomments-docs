## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createQuestionConfigBody | CreateQuestionConfigBody | No |  |

## Respuesta

Devuelve: [`Option[CreateQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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