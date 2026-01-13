## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userId | string | 아니요 |  |
| startDate | string | 아니요 |  |
| questionId | string | 아니요 |  |
| questionIds | string | 아니요 |  |
| skip | float64 | 아니요 |  |

## 응답

반환: [`Option[GetQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results200response.nim)

## 예제

[inline-code-attrs-start title = 'getQuestionResults 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  urlId = "news/politics/election-2024",
  userId = "user-9876",
  startDate = "2024-01-01T00:00:00Z",
  questionId = "q-user-satisfaction",
  questionIds = "q-user-satisfaction,q-engagement",
  skip = 0.0
)

if response.isSome:
  let results = response.get()
  echo "Got question results: ", $results
else:
  echo "No results, HTTP status: ", httpResponse.status
[inline-code-end]