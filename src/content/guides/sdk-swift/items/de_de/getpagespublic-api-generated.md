List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Undurchsichtiger Pagination-Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anfrage. Gebunden an das gleiche `sortBy`. |
| limit | integer | query | No | 1..200, Standard 50 |
| q | string | query | No | Optionaler, nicht case-sensitiver Titel-Präfix-Filter. |
| sortBy | string | query | No | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (die meisten Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | No | Falls true, nur Seiten zurückgeben, die mindestens einen Kommentar enthalten. |

## Response

Rückgabe: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in Beta. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Undurchsichtiger Pagination-Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anfrage. Gebunden an das gleiche `sortBy`. (optional)
let limit = 987 // Int | 1..200, Standard 50 (optional)
let q = "q_example" // String | Optionaler, nicht case-sensitiver Titel-Präfix-Filter. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (die meisten Kommentare zuerst) oder `title` (alphabetisch). (optional)
let hasComments = true // Bool | Falls true, nur Seiten zurückgeben, die mindestens einen Kommentar enthalten. (optional)

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