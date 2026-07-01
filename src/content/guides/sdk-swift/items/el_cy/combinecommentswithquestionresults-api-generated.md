## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|------------|
| tenantId | string | ερώτημα | Ναι |  |
| questionId | string | ερώτημα | Όχι |  |
| questionIds | array | ερώτημα | Όχι |  |
| urlId | string | ερώτημα | Όχι |  |
| startDate | string | ερώτημα | Όχι |  |
| forceRecalculate | boolean | ερώτημα | Όχι |  |
| minValue | number | ερώτημα | Όχι |  |
| maxValue | number | ερώτημα | Όχι |  |
| limit | number | ερώτημα | Όχι |  |

## Απόκριση

Επιστρέφει: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα combineCommentsWithQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (προαιρετικό)
let questionIds = ["inner_example"] // [String] |  (προαιρετικό)
let urlId = "urlId_example" // String |  (προαιρετικό)
let startDate = Date() // Date |  (προαιρετικό)
let forceRecalculate = true // Bool |  (προαιρετικό)
let minValue = 987 // Double |  (προαιρετικό)
let maxValue = 987 // Double |  (προαιρετικό)
let limit = 987 // Double |  (προαιρετικό)

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