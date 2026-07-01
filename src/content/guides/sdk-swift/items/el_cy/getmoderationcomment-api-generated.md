## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| includeEmail | boolean | query | Όχι |  |
| includeIP | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getModerationComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω παραδείγματα κώδικα βρίσκονται ακόμη σε beta. Για οποιοδήποτε ζήτημα, παρακαλώ αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (προαιρετικό)
let includeIP = true // Bool |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

ModerationAPI.getModerationComment(tenantId: tenantId, commentId: commentId, options: ModerationAPI.GetModerationCommentOptions(includeEmail: includeEmail, includeIP: includeIP, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]