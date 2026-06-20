Seznam strani za najemnika. Uporablja se v namiznem odjemalcu FChat za polnjenje seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop skupin zahtevajočega uporabnika.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kurzor za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezen filter po začetku naslova, ki ni občutljiv na velike/majhne črke. |
| sortBy | string | query | Ne | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej), ali `title` (abecedno). |
| hasComments | boolean | query | Ne | Če je true, vrne samo strani z vsaj enim komentarjem. |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za katerokoli težavo, prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozoren kurzor za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. (neobvezno)
let limit = 987 // Int | 1..200, privzeto 50 (neobvezno)
let q = "q_example" // String | Neobvezen filter po začetku naslova, ki ni občutljiv na velike/majhne črke. (neobvezno)
let sortBy = PagesSortBy() // PagesSortBy | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej), ali `title` (abecedno). (neobvezno)
let hasComments = true // Bool | Če je true, vrne samo strani z vsaj enim komentarjem. (neobvezno)

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