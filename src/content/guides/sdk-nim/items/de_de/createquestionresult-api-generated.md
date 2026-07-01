## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| createQuestionResultBody | CreateQuestionResultBody | Nein |  |

## Antwort

Rückgabe: [`Option[CreateQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result_response.nim)

## Beispiel

[inline-code-attrs-start title = 'createQuestionResult Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let body = CreateQuestionResultBody()
let (optResult, httpResp) = client.createQuestionResult(tenantId = tenantId, createQuestionResultBody = body)
if optResult.isSome:
  let result = optResult.get()
  echo result
[inline-code-end]