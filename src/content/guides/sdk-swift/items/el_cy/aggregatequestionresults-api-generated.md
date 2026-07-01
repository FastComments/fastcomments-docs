## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-------------|
| tenantId | string | query | Ναι |  |
| questionId | string | query | Όχι |  |
| questionIds | array | query | Όχι |  |
| urlId | string | query | Όχι |  |
| timeBucket | string | query | Όχι |  |
| startDate | string | query | Όχι |  |
| forceRecalculate | boolean | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResultsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Ο παρακάτω κώδικας είναι ακόμη σε έκδοση beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε το στο http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (προαιρετικό)
let questionIds = ["inner_example"] // [String] |  (προαιρετικό)
let urlId = "urlId_example" // String |  (προαιρετικό)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (προαιρετικό)
let startDate = Date() // Date |  (προαιρετικό)
let forceRecalculate = true // Bool |  (προαιρετικό)

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

---