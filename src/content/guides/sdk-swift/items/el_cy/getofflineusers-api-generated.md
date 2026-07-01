Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL σελίδας (καθαρισμένο στο διακομιστή). |
| afterName | string | query | Όχι | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Δεσμευτής ισότητας κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName είναι ορισμένο ώστε οι ισότητες ονομάτων να μην αφαιρούν εγγραφές. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω παραδείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε ζήτημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Αναγνωριστικό URL σελίδας (καθαρισμένο στο διακομιστή).
let afterName = "afterName_example" // String | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση. (προαιρετικό)
let afterUserId = "afterUserId_example" // String | Δεσμευτής ισότητας κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName είναι ορισμένο ώστε οι ισότητες ονομάτων να μην αφαιρούν εγγραφές. (προαιρετικό)

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

---