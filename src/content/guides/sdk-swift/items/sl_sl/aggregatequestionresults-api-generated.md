## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| questionId | string | query | Ne |  |
| questionIds | array | query | Ne |  |
| urlId | string | query | Ne |  |
| timeBucket | string | query | Ne |  |
| startDate | string | query | Ne |  |
| forceRecalculate | boolean | query | Ne |  |

## Odgovor

Vrača: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResultsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za kakršnekoli težave prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (neobvezno)
let questionIds = ["inner_example"] // [String] |  (neobvezno)
let urlId = "urlId_example" // String |  (neobvezno)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (neobvezno)
let startDate = Date() // Date |  (neobvezno)
let forceRecalculate = true // Bool |  (neobvezno)

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