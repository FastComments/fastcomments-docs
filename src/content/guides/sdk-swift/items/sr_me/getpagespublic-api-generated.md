List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan za isti `sortBy`. |
| limit | integer | query | Ne | 1..200, podrazumijevano 50 |
| q | string | query | Ne | Opcionalni filter prefiksa naslova, neosjetljiv na veličinu slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (podrazumijevano, najnoviji prvo), `commentCount` (najviše komentara prvo), ili `title` (azbučni). |
| hasComments | boolean | query | Ne | Ako je true, vraća se samo stranice koje imaju barem jedan komentar. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozirni kursor za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan za isti `sortBy`. (optional)
let limit = 987 // Int | 1..200, podrazumijevano 50 (optional)
let q = "q_example" // String | Opcionalni filter prefiksa naslova, neosjetljiv na veličinu slova. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Redoslijed sortiranja. `updatedAt` (podrazumijevano, najnoviji prvo), `commentCount` (najviše komentara prvo), ili `title` (azbučni). (optional)
let hasComments = true // Bool | Ako je true, vraća se samo stranice koje imaju barem jedan komentar. (optional)

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