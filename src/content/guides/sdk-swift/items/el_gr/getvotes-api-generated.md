## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetVotes200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 

DefaultAPI.getVotes(tenantId: tenantId, urlId: urlId) { (response, error) in
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