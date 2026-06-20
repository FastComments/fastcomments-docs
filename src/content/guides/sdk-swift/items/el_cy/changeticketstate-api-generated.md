## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ChangeTicketStateResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα changeTicketState'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα παραδείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε ζήτημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String | 
let id = "id_example" // String | 
let changeTicketStateBody = ChangeTicketStateBody(state: 123) // ChangeTicketStateBody | 

DefaultAPI.changeTicketState(tenantId: tenantId, userId: userId, id: id, changeTicketStateBody: changeTicketStateBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]