## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nee |  |
| questionIds | array | query | Nee |  |
| urlId | string | query | Nee |  |
| timeBucket | string | query | Nee |  |
| startDate | string | query | Nee |  |
| forceRecalculate | boolean | query | Nee |  |

## Response

Retourneert: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResultsResponse.swift)

## Example

[inline-code-attrs-start title = 'aggregateQuestionResults Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in beta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (optioneel)
let questionIds = ["inner_example"] // [String] |  (optioneel)
let urlId = "urlId_example" // String |  (optioneel)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (optioneel)
let startDate = Date() // Date |  (optioneel)
let forceRecalculate = true // Bool |  (optioneel)

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