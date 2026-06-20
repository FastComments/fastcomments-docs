## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filter | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za težave prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (neobvezno)
let byIPFromComment = "byIPFromComment_example" // String |  (neobvezno)
let filter = "filter_example" // String |  (neobvezno)
let searchFilters = "searchFilters_example" // String |  (neobvezno)
let demo = true // Bool |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

ModerationAPI.getCount(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]