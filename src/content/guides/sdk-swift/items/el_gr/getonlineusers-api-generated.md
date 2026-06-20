Τρέχοντες συνδεδεμένοι θεατές μιας σελίδας: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή αυτή τη στιγμή.
Επιστρέφει anonCount + totalCount (συνδρομητές σε επίπεδο δωματίου, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Ταυτότητα URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Tiebreaker δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοπαλίες ονομάτων να μην απορρίπτουν εγγραφές. |

## Απάντηση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Ταυτότητα URL της σελίδας (καθαρίζεται από τον διακομιστή).
let afterName = "afterName_example" // String | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση. (προαιρετικό)
let afterUserId = "afterUserId_example" // String | Διαιτητής για τον δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοπαλίες ονομάτων να μην παραλείπουν εγγραφές. (προαιρετικό)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]