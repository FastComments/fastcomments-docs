Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde dens rumsliste.
Kræver, at `enableFChat` er true på den resolved custom config for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nej | Uigennemsigtig pagineringscursor returneret som `nextCursor` fra en tidligere forespørgsel. Bundet til samme `sortBy`. |
| limit | integer | query | Nej | 1..200, standard 50 |
| q | string | query | Nej | Valgfrit titelpræfiksfilter, der ikke skelner mellem store og små bogstaver. |
| sortBy | string | query | Nej | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | Nej | Hvis true, returnér kun sider med mindst én kommentar. |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Uigennemsigtig pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Bundet til samme `sortBy`. (valgfri)
let limit = 987 // Int | 1..200, standard 50 (valgfri)
let q = "q_example" // String | Valgfrit titelpræfiksfilter, der ikke skelner mellem store og små bogstaver. (valgfri)
let sortBy = PagesSortBy() // PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). (valgfri)
let hasComments = true // Bool | Hvis true, returnér kun sider med mindst én kommentar. (valgfri)

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