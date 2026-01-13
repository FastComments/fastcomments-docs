---
## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| urlId | string | query | Όχι |  |
| fromCommentId | string | query | Όχι |  |
| viewed | boolean | query | Όχι |  |
| type | string | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotifications200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (προαιρετικό)
let urlId = "urlId_example" // String |  (προαιρετικό)
let fromCommentId = "fromCommentId_example" // String |  (προαιρετικό)
let viewed = true // Bool |  (προαιρετικό)
let type = "type_example" // String |  (προαιρετικό)
let skip = 987 // Double |  (προαιρετικό)

DefaultAPI.getNotifications(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip) { (response, error) in
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