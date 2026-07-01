List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahteva. Povezan je sa istim `sortBy`. |
| limit | integer | query | Ne | 1..200, podrazumevano 50 |
| q | string | query | Ne | Opcioni filter za prefiks naslova koji ne razlikuje velika i mala slova. |
| sortBy | string | query | Ne | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraća samo stranice koje imaju bar jedan komentar. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozirni kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahteva. Povezan je sa istim `sortBy`. (opcionalno)
let limit = 987 // Int | 1..200, podrazumevano 50 (opcionalno)
let q = "q_example" // String | Opcioni filter za prefiks naslova koji ne razlikuje velika i mala slova. (opcionalno)
let sortBy = PagesSortBy() // PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). (opcionalno)
let hasComments = true // Bool | Ako je true, vraća samo stranice koje imaju bar jedan komentar. (opcionalno)

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