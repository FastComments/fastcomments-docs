## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Ответ

Возвращает: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResultsResponse.swift)

## Пример

[inline-code-attrs-start title = 'aggregateQuestionResults Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (необязательно)
let questionIds = ["inner_example"] // [String] |  (необязательно)
let urlId = "urlId_example" // String |  (необязательно)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (необязательно)
let startDate = Date() // Date |  (необязательно)
let forceRecalculate = true // Bool |  (необязательно)

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