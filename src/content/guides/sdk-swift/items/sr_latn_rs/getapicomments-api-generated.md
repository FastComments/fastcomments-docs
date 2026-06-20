## Parametri

| Name | Type | Location | Required | Description |
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

Vraća: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'getApiComments Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje u beta fazi. Za bilo kakav problem, prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (opciono)
let count = 987 // Double |  (opciono)
let textSearch = "textSearch_example" // String |  (opciono)
let byIPFromComment = "byIPFromComment_example" // String |  (opciono)
let filters = "filters_example" // String |  (opciono)
let searchFilters = "searchFilters_example" // String |  (opciono)
let sorts = "sorts_example" // String |  (opciono)
let demo = true // Bool |  (opciono)
let sso = "sso_example" // String |  (opciono)

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

---