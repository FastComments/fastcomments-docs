---
Lista stranica za tenant. Koristi se od strane FChat desktop klijenta za popunjavanje njegove liste soba. Zahtijeva da `enableFChat` bude true u dobijenoj prilagođenoj konfiguraciji za svaku stranicu. Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji podnosi zahtjev.

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Nečitljiv kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan sa istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (alfabetički). |
| hasComments | boolean | query | Ne | Ako je true, vratiće samo stranice sa najmanje jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Nečitljiv kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan sa istim `sortBy`. (opcionalno)
let limit = 987 // Int | 1..200, zadano 50 (opcionalno)
let q = "q_example" // String | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova. (opcionalno)
let sortBy = PagesSortBy() // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (alfabetički). (opcionalno)
let hasComments = true // Bool | Ako je true, vrati samo stranice sa najmanje jednim komentarom. (opcionalno)

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