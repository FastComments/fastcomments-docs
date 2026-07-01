Τρέχοντες online θεατές μιας σελίδας: άτομα των οποίων η συνεδρία websocket είναι εγγεγραμμένη στη σελίδα αυτή τη στιγμή.  
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων των ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή). |
| afterName | string | query | Όχι | Δρομέας: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Δρομέας-διακόπτης: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί afterName ώστε να μην παραλείπονται εγγραφές λόγω ισοδότησης ονομάτων. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή).
let afterName = "afterName_example" // String | Δρομέας: περάστε το nextAfterName από την προηγούμενη απάντηση. (προαιρετικό)
let afterUserId = "afterUserId_example" // String | Δρομέας-διακόπτης: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί afterName ώστε να μην παραλείπονται εγγραφές λόγω ισοδότησης ονομάτων. (προαιρετικό)

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