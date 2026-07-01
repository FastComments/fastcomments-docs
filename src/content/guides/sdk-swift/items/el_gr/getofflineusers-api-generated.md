Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρισμένο στην πλευρά του διακομιστή). |
| afterName | string | query | No | Δείκτης: περάστε nextAfterName από την προηγούμενη απάντηση. (προαιρετικό) |
| afterUserId | string | query | No | Δεσμευτής ισοπαλίας δείκτη: περάστε nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν afterName ορίζεται ώστε οι ισοτιμίες ονομασιών να μην αφαιρούν εγγραφές. (προαιρετικό) |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για τυχόν πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Αναγνωριστικό URL σελίδας (καθαρισμένο στην πλευρά του διακομιστή).
let afterName = "afterName_example" // String | Δείκτης: περάστε nextAfterName από την προηγούμενη απάντηση. (προαιρετικό)
let afterUserId = "afterUserId_example" // String | Δεσμευτής ισοπαλίας δείκτη: περάστε nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν afterName ορίζεται ώστε οι ισοτιμίες ονομασιών να μην αφαιρούν εγγραφές. (προαιρετικό)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]