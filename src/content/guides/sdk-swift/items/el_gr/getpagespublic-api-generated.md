List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιάφανη δεικτοδότησης (pagination cursor) που επιστρέφεται ως `nextCursor` από ένα προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φιλτράρισμα προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτα τα νεότερα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητική). |
| hasComments | boolean | query | No | Αν true, επιστρέφονται μόνο σελίδες που έχουν τουλάχιστον ένα σχόλιο. |

## Απάντηση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα βρίσκονται ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Αδιάφανη δεικτοδότησης (pagination cursor) που επιστρέφεται ως `nextCursor` από ένα προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. (προαιρετικό)
let limit = 987 // Int | 1..200, προεπιλογή 50 (προαιρετικό)
let q = "q_example" // String | Προαιρετικό φιλτράρισμα προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. (προαιρετικό)
let sortBy = PagesSortBy() // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτα τα νεότερα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητική). (προαιρετικό)
let hasComments = true // Bool | Αν true, επιστρέφονται μόνο σελίδες που έχουν τουλάχιστον ένα σχόλιο. (προαιρετικό)

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