## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | ερώτημα | Ναι |  |
| id | string | διαδρομή | Ναι |  |
| contextUserId | string | ερώτημα | Όχι |  |
| isLive | boolean | ερώτημα | Όχι |  |

## Απόκριση

Επιστρέφει: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteCommentResult.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let id = "id_example" // String |
let contextUserId = "contextUserId_example" // String |  (προαιρετικό)
let isLive = true // Bool |  (προαιρετικό)

DefaultAPI.deleteComment(tenantId: tenantId, id: id, options: DefaultAPI.DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]