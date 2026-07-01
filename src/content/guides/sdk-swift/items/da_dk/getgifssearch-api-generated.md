## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| search | string | query | Ja |  |
| locale | string | query | Nej |  |
| rating | string | query | Nej |  |
| page | number | query | Nej |  |

## Respons

Returnerer: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getGifsSearch Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (valgfri)
let rating = "rating_example" // String |  (valgfri)
let page = 987 // Double |  (valgfri)

PublicAPI.getGifsSearch(tenantId: tenantId, search: search, options: PublicAPI.GetGifsSearchOptions(locale: locale, rating: rating, page: page)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]