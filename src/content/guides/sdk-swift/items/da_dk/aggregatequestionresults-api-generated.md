## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nej |  |
| questionIds | array | query | Nej |  |
| urlId | string | query | Nej |  |
| timeBucket | string | query | Nej |  |
| startDate | string | query | Nej |  |
| forceRecalculate | boolean | query | Nej |  |

## Svar

Returnerer: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'aggregateQuestionResults Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (valgfri)
let questionIds = ["inner_example"] // [String] |  (valgfri)
let urlId = "urlId_example" // String |  (valgfri)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (valgfri)
let startDate = Date() // Date |  (valgfri)
let forceRecalculate = true // Bool |  (valgfri)

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