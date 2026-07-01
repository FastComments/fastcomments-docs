List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Zahtijeva `enableFChat` da bude true na razriješenoj prilagođenoj konfiguraciji za svaku stranicu.  
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji zahtijeva.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan uz isti `sortBy`. |
| limit | integer | query | No | 1..200, zadano 50 |
| q | string | query | No | Opcionalni filter prefiksa naslova neosjetljiv na veličinu slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice s najmanje jednim komentarom. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'Primjer getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još uvijek u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan uz isti `sortBy`. (optional)
let limit = 987 // Int | 1..200, zadano 50 (optional)
let q = "q_example" // String | Opcionalni filter prefiksa naslova neosjetljiv na veličinu slova. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). (optional)
let hasComments = true // Bool | Ako je true, vraća samo stranice s najmanje jednim komentarom. (optional)

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