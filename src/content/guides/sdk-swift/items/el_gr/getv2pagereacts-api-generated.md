## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Όταν οριστεί, επιστρέφει τους χρήστες για κάθε react id (εικονίδια και ονόματα χρηστών). Μόνο το \"public\" γίνεται αποδεκτό, το οποίο παραλείπει τα ιδιωτικά προφίλ. |

## Response

Επιστρέφει: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'getV2PageReacts Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (προαιρετικό)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Όταν οριστεί, επιστρέφει τους χρήστες για κάθε react id (εικονίδια και ονόματα χρηστών). Μόνο το \"public\" γίνεται αποδεκτό, το οποίο παραλείπει τα ιδιωτικά προφίλ. (προαιρετικό)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]