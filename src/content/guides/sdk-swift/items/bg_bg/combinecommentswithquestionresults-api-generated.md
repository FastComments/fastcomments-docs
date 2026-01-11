## Параметри

| Име | Type | Location | Required | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |
| minValue | number | query | No |  |
| maxValue | number | query | No |  |
| limit | number | query | No |  |

## Отговор

Връща: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineCommentsWithQuestionResults200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример за combineCommentsWithQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (незадължително)
let questionIds = ["inner_example"] // [String] |  (незадължително)
let urlId = "urlId_example" // String |  (незадължително)
let startDate = Date() // Date |  (незадължително)
let forceRecalculate = true // Bool |  (незадължително)
let minValue = 987 // Double |  (незадължително)
let maxValue = 987 // Double |  (незадължително)
let limit = 987 // Double |  (незадължително)

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