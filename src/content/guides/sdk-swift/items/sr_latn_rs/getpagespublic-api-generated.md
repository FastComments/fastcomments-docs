Lista stranica za tenanta. Koristi ga FChat desktop klijent za popunjavanje svoje liste soba.
Zahteva da `enableFChat` bude true na rešavanom prilagođenom podešavanju za svaku stranicu.
Stranice koje zahtevaju SSO filtriraju se u skladu sa pristupom grupa korisnika koji šalje zahtev.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Nečitljiv kursor paginacije koji se vraća kao `nextCursor` iz prethodnog zahteva. Povezan sa istim `sortBy`. |
| limit | integer | query | Ne | 1..200, podrazumevano 50 |
| q | string | query | Ne | Opcioni filter po prefiksu naslova koji nije osetljiv na velika/mala slova. |
| sortBy | string | query | Ne | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vrati samo stranice sa najmanje jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo kakav problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Nečitljiv kursor paginacije koji se vraća kao `nextCursor` iz prethodnog zahteva. Povezan sa istim `sortBy`. (opciono)
let limit = 987 // Int | 1..200, podrazumevano 50 (opciono)
let q = "q_example" // String | Opcioni filter po prefiksu naslova koji nije osetljiv na velika/mala slova. (opciono)
let sortBy = PagesSortBy() // PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). (opciono)
let hasComments = true // Bool | Ako je true, vrati samo stranice sa bar jednim komentarom. (opciono)

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