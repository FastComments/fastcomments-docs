## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| questionId | string | query | Όχι |  |
| questionIds | array | query | Όχι |  |
| urlId | string | query | Όχι |  |
| startDate | string | query | Όχι |  |
| forceRecalculate | boolean | query | Όχι |  |
| minValue | number | query | Όχι |  |
| maxValue | number | query | Όχι |  |
| limit | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineCommentsWithQuestionResults200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα combineCommentsWithQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
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

DefaultAPI.combineCommentsWithQuestionResults(tenantId: tenantId, questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]