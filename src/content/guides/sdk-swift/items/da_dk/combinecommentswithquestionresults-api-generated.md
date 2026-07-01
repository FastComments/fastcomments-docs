## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | forespørgsel | Ja |  |
| questionId | string | forespørgsel | Nej |  |
| questionIds | array | forespørgsel | Nej |  |
| urlId | string | forespørgsel | Nej |  |
| startDate | string | forespørgsel | Nej |  |
| forceRecalculate | boolean | forespørgsel | Nej |  |
| minValue | number | forespørgsel | Nej |  |
| maxValue | number | forespørgsel | Nej |  |
| limit | number | forespørgsel | Nej |  |

## Svar

Returnerer: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (valgfri)
let questionIds = ["inner_example"] // [String] |  (valgfri)
let urlId = "urlId_example" // String |  (valgfri)
let startDate = Date() // Date |  (valgfri)
let forceRecalculate = true // Bool |  (valgfri)
let minValue = 987 // Double |  (valgfri)
let maxValue = 987 // Double |  (valgfri)
let limit = 987 // Double |  (valgfri)

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