Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον FChat desktop client για τη συμπλήρωση της λίστας δωματίων του.
Απαιτεί το `enableFChat` να είναι true στο resolved custom config για κάθε σελίδα.
Σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| cursor | string | query | Όχι | Αδιαφανές pagination cursor που επιστράφηκε ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`. |
| limit | integer | query | Όχι | 1..200, προεπιλογή 50 |
| q | string | query | Όχι | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων. |
| sortBy | string | query | Όχι | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | Όχι | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απάντηση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Αδιαφανές pagination cursor που επιστράφηκε ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`. (optional)
let limit = 987 // Int | 1..200, προεπιλογή 50 (optional)
let q = "q_example" // String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητικά). (optional)
let hasComments = true // Bool | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]