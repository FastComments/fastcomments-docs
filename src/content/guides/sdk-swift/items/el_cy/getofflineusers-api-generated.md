Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή συνδεδεμένοι. Ταξινομημένα κατά displayName.
Χρησιμοποιήστε αυτό αφού εξαντλήσετε το /users/online για να εμφανίσετε μια ενότητα «Μέλη».
Σελιδοποίηση με cursor στο commenterName: ο διακομιστής διασχίζει τον μερικό δείκτη {tenantId, urlId, commenterName} από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | Όχι | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοβαθμίες ονομάτων να μην παραλείψουν εγγραφές. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον διακομιστή).
let afterName = "afterName_example" // String | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απόκριση. (προαιρετικό)
let afterUserId = "afterUserId_example" // String | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοβαθμίες ονομάτων να μην παραλείψουν εγγραφές. (προαιρετικό)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]