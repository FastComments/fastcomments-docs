List sider for en lejer. Brugt af FChat desktopklienten til at udfylde dens rumliste.
Kræver `enableFChat` skal være sand på den opløste brugerdefinerede konfiguration for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Uigennemsigtig pagineringsmarkør returneret som `nextCursor` fra en tidligere anmodning. Bundet til den samme `sortBy`. |
| limit | integer | query | No | 1..200, standard 50 |
| q | string | query | No | Valgfri case‑insensitiv titelpræfiksfilter. |
| sortBy | string | query | No | Sorteringsorden. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | No | Hvis sand, returneres kun sider med mindst én kommentar. |

## Svar

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Ved problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Uigennemsigtig pagineringsmarkør returneret som `nextCursor` fra en tidligere anmodning. Bundet til den samme `sortBy`. (optional)
let limit = 987 // Int | 1..200, standard 50 (optional)
let q = "q_example" // String | Valgfri case‑insensitiv titelpræfiksfilter. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Sorteringsorden. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk). (optional)
let hasComments = true // Bool | Hvis sand, returneres kun sider med mindst én kommentar. (optional)

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