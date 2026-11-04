Αριθμός likes για μια σελίδα, προαιρετικά με τους χρήστες που έκαναν like.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Ναι |  |
| includeUserInfo | string | query | Όχι | Το "public" παραλείπει χρήστες με ιδιωτικά προφίλ, το "include-private" τους συμπεριλαμβάνει. |

## Απάντηση

Επιστρέφει: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageLikesAPIResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Το \"public\" παραλείπει χρήστες με ιδιωτικά προφίλ, το \"include-private\" τους συμπεριλαμβάνει. (optional)

DefaultAPI.getPageLikes(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]