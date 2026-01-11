## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nein |  |
| questionIds | array | query | Nein |  |
| urlId | string | query | Nein |  |
| timeBucket | string | query | Nein |  |
| startDate | string | query | Nein |  |
| forceRecalculate | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## Beispiel

[inline-code-attrs-start title = 'aggregateQuestionResults Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (optional)
let questionIds = ["inner_example"] // [String] |  (optional)
let urlId = "urlId_example" // String |  (optional)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (optional)
let startDate = Date() // Date |  (optional)
let forceRecalculate = true // Bool |  (optional)

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