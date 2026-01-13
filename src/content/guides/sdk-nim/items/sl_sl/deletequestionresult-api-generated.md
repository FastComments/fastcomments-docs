## Parametri

| Name | Type | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vrne: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer deleteQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionResult(tenantId = "my-tenant-123", id = "question-result-789")
if response.isSome:
  let result = response.get()
  echo "Deleted question result:", result
else:
  echo "No result returned, HTTP status:", $httpResponse.status
[inline-code-end]

---