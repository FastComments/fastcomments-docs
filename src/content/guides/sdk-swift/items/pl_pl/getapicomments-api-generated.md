## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| page | number | query | Nie |  |
| count | number | query | Nie |  |
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sorts | string | query | Nie |  |
| demo | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (opcjonalne)
let count = 987 // Double |  (opcjonalne)
let textSearch = "textSearch_example" // String |  (opcjonalne)
let byIPFromComment = "byIPFromComment_example" // String |  (opcjonalne)
let filters = "filters_example" // String |  (opcjonalne)
let searchFilters = "searchFilters_example" // String |  (opcjonalne)
let sorts = "sorts_example" // String |  (opcjonalne)
let demo = true // Bool |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

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