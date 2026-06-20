---
Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον πελάτη επιφάνειας εργασίας FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτεί το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδων του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| cursor | string | query | Όχι | Αδιαφανές pagination cursor που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | Όχι | 1..200, προεπιλογή 50 |
| q | string | query | Όχι | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | Όχι | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | Όχι | Αν είναι true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. (προαιρετικό)
let limit = 987 // Int | 1..200, προεπιλογή 50 (προαιρετικό)
let q = "q_example" // String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. (προαιρετικό)
let sortBy = PagesSortBy() // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). (προαιρετικό)
let hasComments = true // Bool | Αν είναι true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. (προαιρετικό)

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

---