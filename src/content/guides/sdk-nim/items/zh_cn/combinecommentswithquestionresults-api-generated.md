---
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

返回：[`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## 示例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "",
  questionIds = @[],
  urlId = "news/article-2026-climate-change",
  startDate = "",
  forceRecalculate = false,
  minValue = 0.0,
  maxValue = 0.0,
  limit = 0.0
)

if response.isSome:
  let combined = response.get()
  echo "Combined results received for tenant:", " my-tenant-123"
else:
  echo "No combined results returned"
[inline-code-end]

---