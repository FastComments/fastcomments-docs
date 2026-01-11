## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Risposta

Restituisce: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalalo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (opzionale)
let questionIds = ["inner_example"] // [String] |  (opzionale)
let urlId = "urlId_example" // String |  (opzionale)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (opzionale)
let startDate = Date() // Date |  (opzionale)
let forceRecalculate = true // Bool |  (opzionale)

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