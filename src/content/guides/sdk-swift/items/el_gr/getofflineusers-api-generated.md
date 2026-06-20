---
Προηγούμενοι σχολιαστές στη σελίδα που δεν είναι αυτή τη στιγμή συνδεδεμένοι. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε αυτό μετά την εξάντληση του /users/online για να εμφανίσετε μια ενότητα "Μέλη".
Σελιδοποίηση με δείκτη (cursor) στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName} από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Tiebreaker δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαραίτητο όταν το afterName έχει οριστεί ώστε δεσμοί ονομάτων να μην παραλείπονται. |

## Response

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον διακομιστή).
let afterName = "afterName_example" // String | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απάντηση. (προαιρετικό)
let afterUserId = "afterUserId_example" // String | Tiebreaker δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαραίτητο όταν το afterName έχει οριστεί ώστε δεσμοί ονομάτων να μην παραλείπονται. (προαιρετικό)

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

---