## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | GetQuestionResultsOptions | 否 |  |

## 响应

返回：[`Option[GetQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results_response.nim)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  options = GetQuestionResultsOptions()
)

if response.isSome:
  let results = response.get()
  echo results
[inline-code-end]