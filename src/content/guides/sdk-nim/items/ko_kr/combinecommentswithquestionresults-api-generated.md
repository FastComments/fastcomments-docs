---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| options | CombineCommentsWithQuestionResultsOptions | No |  |

## 응답

반환: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## 예시

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (combineOpt, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  options = default(CombineCommentsWithQuestionResultsOptions)
)

if combineOpt.isSome:
  let combineResult = combineOpt.get()
[inline-code-end]

---