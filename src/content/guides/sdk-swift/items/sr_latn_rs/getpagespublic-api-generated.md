List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor za paginaciju vraćen kao `nextCursor` iz prethodnog zahteva. Vezan za isti `sortBy`. |
| limit | integer | query | Ne | 1..200, podrazumevano 50 |
| q | string | query | Ne | Opcioni filter prefiksa naslova, neosetljiv na velika i mala slova. |
| sortBy | string | query | Ne | Redosled sortiranja. `updatedAt` (podrazumevano, prvo najnoviji), `commentCount` (prvo oni sa najviše komentara) ili `title` (alfabetski). |
| hasComments | boolean | query | Ne | Ako je true, vraća samo stranice sa bar jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primer

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozirni kursor za paginaciju vraćen kao `nextCursor` iz prethodnog zahteva. Vezan za isti `sortBy`. (optional)
let limit = 987 // Int | 1..200, podrazumevano 50 (optional)
let q = "q_example" // String | Opcioni filter prefiksa naslova, neosetljiv na velika i mala slova. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, prvo najnoviji), `commentCount` (prvo oni sa najviše komentara) ili `title` (alfabetski). (optional)
let hasComments = true // Bool | Ako je true, vraća samo stranice sa bar jednim komentarom. (optional)

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