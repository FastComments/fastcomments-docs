Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον desktop client FChat για να γεμίσει τη λίστα δωματίων του.
Απαιτείται το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση σε ομάδες του αιτούμενου χρήστη.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής δρομέας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με την ίδια `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου ανεξαρτήτως πεζών/κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Αν είναι true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Αδιαφανής δρομέας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με την ίδια `sortBy`. (προαιρετικό)
let limit = 987 // Int | 1..200, προεπιλογή 50 (προαιρετικό)
let q = "q_example" // String | Προαιρετικό φίλτρο προθέματος τίτλου ανεξαρτήτως πεζών/κεφαλαίων. (προαιρετικό)
let sortBy = PagesSortBy() // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). (προαιρετικό)
let hasComments = true // Bool | Αν είναι true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. (προαιρετικό)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]