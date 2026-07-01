## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|--------------|----------|------|
| tenantId | string | query | Tak |  |
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

[inline-code-attrs-start title = 'getApiIds Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe próbki kodu są nadal w wersji beta. W razie jakichkolwiek problemów prosimy zgłaszać je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let textSearch = "textSearch_example" // String |  (opcjonalnie)
let byIPFromComment = "byIPFromComment_example" // String |  (opcjonalnie)
let filters = "filters_example" // String |  (opcjonalnie)
let searchFilters = "searchFilters_example" // String |  (opcjonalnie)
let afterId = "afterId_example" // String |  (opcjonalnie)
let demo = true // Bool |  (opcjonalnie)
let sso = "sso_example" // String |  (opcjonalnie)

ModerationAPI.getApiIds(tenantId: tenantId, options: ModerationAPI.GetApiIdsOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]