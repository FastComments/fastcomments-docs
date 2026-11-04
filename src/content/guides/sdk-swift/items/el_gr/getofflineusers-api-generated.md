Προηγούμενοι σχολιαστές της σελίδας που ΔΕΝ είναι αυτή τη στιγμή συνδεδεμένοι. Ταξινομούνται κατά displayName.
Χρησιμοποιήστε το αυτό αφού εξαντλήσετε το /users/online για να αποδώσετε μια ενότητα "Members".
Σελιδοποίηση με cursor στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName}
από το afterName και μετά μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν ορίζεται το afterName ώστε οι ισοβαθμίες ονομάτων να μην απορρίπτουν εγγραφές. |

## Απάντηση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το στο http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
let afterName = "afterName_example" // String | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. (optional)
let afterUserId = "afterUserId_example" // String | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν ορίζεται το afterName ώστε οι ισοβαθμίες ονομάτων να μην απορρίπτουν εγγραφές. (optional)

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