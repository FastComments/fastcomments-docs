## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| page | number | query | Ne |  |
| count | number | query | Ne |  |
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| sorts | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za kakršnokoli težavo poročajte prek http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (neobvezno)
let count = 987 // Double |  (neobvezno)
let textSearch = "textSearch_example" // String |  (neobvezno)
let byIPFromComment = "byIPFromComment_example" // String |  (neobvezno)
let filters = "filters_example" // String |  (neobvezno)
let searchFilters = "searchFilters_example" // String |  (neobvezno)
let sorts = "sorts_example" // String |  (neobvezno)
let demo = true // Bool |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

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