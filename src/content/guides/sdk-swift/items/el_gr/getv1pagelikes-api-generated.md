## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| sso | string | query | Όχι |  |
| includeUserInfo | string | query | Όχι | Όταν οριστεί, επιστρέφει τους χρήστες που έδωσαν like στη σελίδα (εικονοστοιχεία και ονόματα χρηστών). Μόνο το \"public\" γίνεται αποδεκτό, το οποίο παραλείπει τα ιδιωτικά προφίλ. |

## Απάντηση

Επιστρέφει: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getV1PageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε να το αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Όταν οριστεί, επιστρέφει τους χρήστες που έδωσαν like στη σελίδα (εικονοστοιχεία και ονόματα χρηστών). Μόνο το \"public\" γίνεται αποδεκτό, το οποίο παραλείπει τα ιδιωτικά προφίλ. (optional)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]