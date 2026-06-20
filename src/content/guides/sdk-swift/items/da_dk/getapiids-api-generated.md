## Parametre

| Name | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| afterId | string | query | Nej |  |
| demo | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getApiIds Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig beta. Ved problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (valgfri)
let byIPFromComment = "byIPFromComment_example" // String |  (valgfri)
let filters = "filters_example" // String |  (valgfri)
let searchFilters = "searchFilters_example" // String |  (valgfri)
let afterId = "afterId_example" // String |  (valgfri)
let demo = true // Bool |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

ModerationAPI.getApiIds(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]