## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| questionId | string | query | 否 |  |
| questionIds | array | query | 否 |  |
| urlId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| forceRecalculate | boolean | query | 否 |  |
| minValue | number | query | 否 |  |
| maxValue | number | query | 否 |  |
| limit | number | query | 否 |  |

## 响应

返回: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## 示例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。若有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (可选)
let questionIds = ["inner_example"] // [String] |  (可选)
let urlId = "urlId_example" // String |  (可选)
let startDate = Date() // Date |  (可选)
let forceRecalculate = true // Bool |  (可选)
let minValue = 987 // Double |  (可选)
let maxValue = 987 // Double |  (可选)
let limit = 987 // Double |  (可选)

DefaultAPI.combineCommentsWithQuestionResults(tenantId: tenantId, questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---