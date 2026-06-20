## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još uvijek u beta fazi. Za bilo koji problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (neobavezno)
let count = 987 // Double |  (neobavezno)
let textSearch = "textSearch_example" // String |  (neobavezno)
let byIPFromComment = "byIPFromComment_example" // String |  (neobavezno)
let filters = "filters_example" // String |  (neobavezno)
let searchFilters = "searchFilters_example" // String |  (neobavezno)
let sorts = "sorts_example" // String |  (neobavezno)
let demo = true // Bool |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

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