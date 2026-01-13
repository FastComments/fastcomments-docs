## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userId | string | 否 |  |
| startDate | string | 否 |  |
| questionId | string | 否 |  |
| questionIds | string | 否 |  |
| skip | float64 | 否 |  |

## 响应

返回：[`Option[GetQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results200response.nim)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---