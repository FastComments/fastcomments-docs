## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Odgovor

Returns: [`Option[GetQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_result_response.nim)

## Primjer

[inline-code-attrs-start title = 'getQuestionResult Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResult, httpResp) = client.getQuestionResult(tenantId = "my-tenant-123", id = "question-456")
if optResult.isSome:
  let result = optResult.get()
  discard result
[inline-code-end]