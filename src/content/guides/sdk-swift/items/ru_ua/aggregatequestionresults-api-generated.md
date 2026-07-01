## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Відповідь

Повертає: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResultsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'aggregateQuestionResults Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще у бета-версії. Якщо виникнуть проблеми, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (опціонально)
let questionIds = ["inner_example"] // [String] |  (опціонально)
let urlId = "urlId_example" // String |  (опціонально)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (опціонально)
let startDate = Date() // Date |  (опціонально)
let forceRecalculate = true // Bool |  (опціонально)

DefaultAPI.aggregateQuestionResults(tenantId: tenantId, options: DefaultAPI.AggregateQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, timeBucket: timeBucket, startDate: startDate, forceRecalculate: forceRecalculate)) { (response, error) in
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