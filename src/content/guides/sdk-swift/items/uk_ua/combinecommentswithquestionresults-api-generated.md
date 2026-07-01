## Parameters

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | запит | Так |  |
| questionId | string | запит | Ні |  |
| questionIds | array | запит | Ні |  |
| urlId | string | запит | Ні |  |
| startDate | string | запит | Ні |  |
| forceRecalculate | boolean | запит | Ні |  |
| minValue | number | запит | Ні |  |
| maxValue | number | запит | Ні |  |
| limit | number | запит | Ні |  |

## Response

Повертає: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## Example

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще у бета-версії. У випадку проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (необов'язково)
let questionIds = ["inner_example"] // [String] |  (необов'язково)
let urlId = "urlId_example" // String |  (необов'язково)
let startDate = Date() // Date |  (необов'язково)
let forceRecalculate = true // Bool |  (необов'язково)
let minValue = 987 // Double |  (необов'язково)
let maxValue = 987 // Double |  (необов'язково)
let limit = 987 // Double |  (необов'язково)

DefaultAPI.combineCommentsWithQuestionResults(tenantId: tenantId, options: DefaultAPI.CombineCommentsWithQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit)) { (response, error) in
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