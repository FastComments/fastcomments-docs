## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| page | number | query | Nej |  |
| count | number | query | Nej |  |
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sorts | string | query | Nej |  |
| demo | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getApiComments Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (valgfri)
let count = 987 // Double |  (valgfri)
let textSearch = "textSearch_example" // String |  (valgfri)
let byIPFromComment = "byIPFromComment_example" // String |  (valgfri)
let filters = "filters_example" // String |  (valgfri)
let searchFilters = "searchFilters_example" // String |  (valgfri)
let sorts = "sorts_example" // String |  (valgfri)
let demo = true // Bool |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

ModerationAPI.getApiComments(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]