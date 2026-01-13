## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Non |  |

## Response

Renvoie : [`Option[CreateQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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