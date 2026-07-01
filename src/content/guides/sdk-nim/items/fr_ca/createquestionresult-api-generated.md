## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | No |  |

## Réponse

Renvoie : [`Option[CreateQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let body = CreateQuestionResultBody()
let (optResult, httpResp) = client.createQuestionResult(tenantId = tenantId, createQuestionResultBody = body)
if optResult.isSome:
  let result = optResult.get()
  echo result
[inline-code-end]