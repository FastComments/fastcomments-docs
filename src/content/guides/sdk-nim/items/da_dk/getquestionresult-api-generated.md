## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |

## Svar

Returns: [`Option[GetQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_result_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getQuestionResult Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResult, httpResp) = client.getQuestionResult(tenantId = "my-tenant-123", id = "question-456")
if optResult.isSome:
  let result = optResult.get()
  discard result
[inline-code-end]