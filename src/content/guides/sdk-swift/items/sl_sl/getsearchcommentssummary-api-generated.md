## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getSearchCommentsSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta. Za kakršnokoli težavo, prosimo, prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (izbirno)
let filters = "filters_example" // String |  (izbirno)
let searchFilters = "searchFilters_example" // String |  (izbirno)
let sso = "sso_example" // String |  (izbirno)

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