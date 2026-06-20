## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getSearchCommentsSummary Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (valgfri)
let filters = "filters_example" // String |  (valgfri)
let searchFilters = "searchFilters_example" // String |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

ModerationAPI.getSearchCommentsSummary(value: value, filters: filters, searchFilters: searchFilters, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]