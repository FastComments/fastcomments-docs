Popis stranica za tenant. Koristi ga FChat desktop klijent za popunjavanje popisa soba.
Zahtijeva da `enableFChat` bude true u riješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Neobavezni filtar prefiksa naslova neosjetljiv na velika/mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraćaju se samo stranice s barem jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. (opcionalno)
let limit = 987 // Int | 1..200, zadano 50 (opcionalno)
let q = "q_example" // String | Neobavezni filtar prefiksa naslova neosjetljiv na velika/mala slova. (opcionalno)
let sortBy = PagesSortBy() // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). (opcionalno)
let hasComments = true // Bool | Ako je true, vraćaju se samo stranice s barem jednim komentarom. (opcionalno)

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