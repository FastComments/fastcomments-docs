Navede strani za najemnika. Uporablja namizni odjemalec FChat za polnjenje svojega seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, se filtrirajo glede na dostop do skupin zahtevajočega uporabnika.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kazalec straniščenja, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. |
| sortBy | string | query | Ne | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). |
| hasComments | boolean | query | Ne | Če je true, vrni samo strani z vsaj enim komentarjem. |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave jih prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Neprozoren kazalec straniščenja, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. (optional)
let limit = 987 // Int | 1..200, privzeto 50 (optional)
let q = "q_example" // String | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). (optional)
let hasComments = true // Bool | Če je true, vrni samo strani z vsaj enim komentarjem. (optional)

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