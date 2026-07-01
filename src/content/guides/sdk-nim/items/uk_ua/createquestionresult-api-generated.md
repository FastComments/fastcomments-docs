## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | No |  |

## Відповідь

Повертає: [`Option[CreateQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result_response.nim)

## Приклад

[inline-code-attrs-start title = 'createQuestionResult Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let body = CreateQuestionResultBody()
let (optResult, httpResp) = client.createQuestionResult(tenantId = tenantId, createQuestionResultBody = body)
if optResult.isSome:
  let result = optResult.get()
  echo result
[inline-code-end]