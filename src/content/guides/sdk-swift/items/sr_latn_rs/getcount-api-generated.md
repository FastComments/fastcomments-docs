## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filter | string | query | No |  |
| searchFilters | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem prijavite ga na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (opciono)
let byIPFromComment = "byIPFromComment_example" // String |  (opciono)
let filter = "filter_example" // String |  (opciono)
let searchFilters = "searchFilters_example" // String |  (opciono)
let demo = true // Bool |  (opciono)
let sso = "sso_example" // String |  (opciono)

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