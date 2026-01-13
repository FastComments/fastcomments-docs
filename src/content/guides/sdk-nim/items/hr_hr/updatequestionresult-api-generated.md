## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Ne |  |

## Odgovor

Vraća: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer updateQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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