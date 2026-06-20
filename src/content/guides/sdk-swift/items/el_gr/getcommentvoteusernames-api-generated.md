## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| dir | integer | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentVoteUserNamesSuccessResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'getCommentVoteUserNames Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let dir = 987 // Int | 
let sso = "sso_example" // String |  (προαιρετικό)

PublicAPI.getCommentVoteUserNames(tenantId: tenantId, commentId: commentId, dir: dir, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]