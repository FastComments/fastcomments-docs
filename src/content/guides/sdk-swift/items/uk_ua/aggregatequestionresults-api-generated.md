## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| questionId | string | query | Ні |  |
| questionIds | array | query | Ні |  |
| urlId | string | query | Ні |  |
| timeBucket | string | query | Ні |  |
| startDate | string | query | Ні |  |
| forceRecalculate | boolean | query | Ні |  |

## Відповідь

Повертає: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще в бета-версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (необов'язково)
let questionIds = ["inner_example"] // [String] |  (необов'язково)
let urlId = "urlId_example" // String |  (необов'язково)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (необов'язково)
let startDate = Date() // Date |  (необов'язково)
let forceRecalculate = true // Bool |  (необов'язково)

DefaultAPI.aggregateQuestionResults(tenantId: tenantId, questionId: questionId, questionIds: questionIds, urlId: urlId, timeBucket: timeBucket, startDate: startDate, forceRecalculate: forceRecalculate) { (response, error) in
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