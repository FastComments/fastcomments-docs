Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον server). |
| afterName | string | query | No | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. (προαιρετικό) |
| afterUserId | string | query | No | Διευθυντής ισοσταθμισμού δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν ορίζεται afterName ώστε τα ισοδυνάμει ονόματα να μην αφαιρούν εγγραφές. (προαιρετικό) |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον server).
let afterName = "afterName_example" // String | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. (προαιρετικό)
let afterUserId = "afterUserId_example" // String | Διευθυντής ισοσταθμισμού δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν ορίζεται afterName ώστε τα ισοδυνάμει ονόματα να μην αφαιρούν εγγραφές. (προαιρετικό)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]