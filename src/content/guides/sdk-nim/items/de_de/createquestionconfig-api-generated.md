## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Nein |  |

## Antwort

Gibt zurück: [`Option[CreateQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config200response.nim)

## Beispiel

[inline-code-attrs-start title = 'createQuestionConfig Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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