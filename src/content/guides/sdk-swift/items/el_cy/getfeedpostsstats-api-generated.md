## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| postIds | array | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FeedPostsStatsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getFeedPostsStats'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω παραδείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] | 
let sso = "sso_example" // String |  (optional)

PublicAPI.getFeedPostsStats(tenantId: tenantId, postIds: postIds, sso: sso) { (response, error) in
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