## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| questionId | string | 否 |  |
| questionIds | seq[string] | 否 |  |
| urlId | string | 是 |  |
| startDate | string | 否 |  |
| forceRecalculate | bool | 否 |  |
| minValue | float64 | 否 |  |
| maxValue | float64 | 否 |  |
| limit | float64 | 否 |  |

## 响应

返回: [`Option[CombineCommentsWithQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_comments_with_question_results200response.nim)

## 示例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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