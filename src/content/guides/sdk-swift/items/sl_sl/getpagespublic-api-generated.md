List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozoren kurzor za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | No | 1..200, privzeto 50 |
| q | string | query | No | Neobvezni neobčutljiv na velikost črk filter predpone naslova. |
| sortBy | string | query | No | Vrstni red. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). |
| hasComments | boolean | query | No | Če je true, vrne le strani z vsaj enim komentarjem. |

## Response

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še beta. Če imate kakršnekoli težave, jih prosim prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozoren kurzor za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. (optional)
let limit = 987 // Int | 1..200, privzeto 50 (optional)
let q = "q_example" // String | Neobvezni neobčutljiv na velikost črk filter predpone naslova. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Vrstni red. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej), ali `title` (abecedno). (optional)
let hasComments = true // Bool | Če je true, vrne le strani z vsaj enim komentarjem. (optional)

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

---