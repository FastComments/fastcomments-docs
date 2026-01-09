## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니오 |  |
| questionIds | seq[string] | 아니오 |  |
| urlId | string | 예 |  |
| startDate | string | 아니오 |  |
| forceRecalculate | bool | 아니오 |  |
| minValue | float64 | 아니오 |  |
| maxValue | float64 | 아니오 |  |
| limit | float64 | 아니오 |  |

## 응답

반환: [`Option[CombineCommentsWithQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_comments_with_question_results200response.nim)

## 예제

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "q-9876",
  questionIds = @["q-9876", "q-9877"],
  urlId = "news/article-title",
  startDate = "2025-01-01T00:00:00Z",
  forceRecalculate = false,
  minValue = 1.0,
  maxValue = 5.0,
  limit = 100.0
)

if response.isSome:
  let combined = response.get()
  discard combined
[inline-code-end]

---