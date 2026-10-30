## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteV1PageReact'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)

PublicAPI.deleteV1PageReact(tenantId: tenantId, urlId: urlId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]