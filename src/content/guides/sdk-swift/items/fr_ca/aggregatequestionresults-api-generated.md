## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| questionId | string | query | Non |  |
| questionIds | array | query | Non |  |
| urlId | string | query | Non |  |
| timeBucket | string | query | Non |  |
| startDate | string | query | Non |  |
| forceRecalculate | boolean | query | Non |  |

## Réponse

Renvoie: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (optionnel)
let questionIds = ["inner_example"] // [String] |  (optionnel)
let urlId = "urlId_example" // String |  (optionnel)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (optionnel)
let startDate = Date() // Date |  (optionnel)
let forceRecalculate = true // Bool |  (optionnel)

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