## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| urlId | string | 查询 | 否 |  |
| userId | string | 查询 | 否 |  |
| startDate | string | 查询 | 否 |  |
| questionId | string | 查询 | 否 |  |
| questionIds | string | 查询 | 否 |  |
| skip | number | 查询 | 否 |  |

## 响应

返回: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (可选)
let userId = "userId_example" // String |  (可选)
let startDate = "startDate_example" // String |  (可选)
let questionId = "questionId_example" // String |  (可选)
let questionIds = "questionIds_example" // String |  (可选)
let skip = 987 // Double |  (可选)

DefaultAPI.getQuestionResults(tenantId: tenantId, urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]