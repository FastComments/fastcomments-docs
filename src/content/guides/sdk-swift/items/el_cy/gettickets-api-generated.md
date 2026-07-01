## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| state | number | query | Όχι |  |
| skip | number | query | Όχι |  |
| limit | number | query | Όχι |  |

## Απόκριση

Returns: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω παραδείγματα κώδικα βρίσκονται ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (προαιρετικό)
let state = 987 // Double |  (προαιρετικό)
let skip = 987 // Double |  (προαιρετικό)
let limit = 987 // Double |  (προαιρετικό)

DefaultAPI.getTickets(tenantId: tenantId, options: DefaultAPI.GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]