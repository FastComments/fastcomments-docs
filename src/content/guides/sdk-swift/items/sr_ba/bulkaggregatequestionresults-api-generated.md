## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| forceRecalculate | boolean | query | Не |  |

## Одговор

Враћа: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkAggregateQuestionResults200Response.swift)

## Пример

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода још су у бета фази. За било који проблем, пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(aggregations: [BulkAggregateQuestionItem(aggId: "aggId_example", questionId: "questionId_example", questionIds: ["questionIds_example"], urlId: "urlId_example", timeBucket: AggregateTimeBucket(), startDate: Date())]) // BulkAggregateQuestionResultsRequest | 
let forceRecalculate = true // Bool |  (опционо)

DefaultAPI.bulkAggregateQuestionResults(tenantId: tenantId, bulkAggregateQuestionResultsRequest: bulkAggregateQuestionResultsRequest, forceRecalculate: forceRecalculate) { (response, error) in
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