List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής κέρσορας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, τα νεότερα πρώτα), `commentCount` (τα περισσότερα σχόλια πρώτα), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Αν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα σε έκδοση beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Αδιαφανής κέρσορας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`. (optional)
let limit = 987 // Int | 1..200, default 50 (optional)
let q = "q_example" // String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, τα νεότερα πρώτα), `commentCount` (τα περισσότερα σχόλια πρώτα), ή `title` (αλφαβητικά). (optional)
let hasComments = true // Bool | Αν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. (optional)

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