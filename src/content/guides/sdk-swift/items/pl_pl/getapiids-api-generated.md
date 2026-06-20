## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| afterId | string | query | Nie |  |
| demo | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getApiIds'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (opcjonalne)
let byIPFromComment = "byIPFromComment_example" // String |  (opcjonalne)
let filters = "filters_example" // String |  (opcjonalne)
let searchFilters = "searchFilters_example" // String |  (opcjonalne)
let afterId = "afterId_example" // String |  (opcjonalne)
let demo = true // Bool |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

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