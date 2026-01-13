## Параметри

| Назва | Type | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Ні |  |

## Відповідь

Повертає: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionResult(
  tenantId = "my-tenant-123",
  id = "question-456",
  updateQuestionResultBody = UpdateQuestionResultBody(
    result = "approved",
    reviewerId = "moderator-42",
    notes = "Valid question, no action required",
    isSpam = false
  )
)
if response.isSome:
  let flagResponse = response.get()
  discard flagResponse
[inline-code-end]

---