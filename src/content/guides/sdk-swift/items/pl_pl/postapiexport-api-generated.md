## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sorts | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemu zgłoś to przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (opcjonalny)
let byIPFromComment = "byIPFromComment_example" // String |  (opcjonalny)
let filters = "filters_example" // String |  (opcjonalny)
let searchFilters = "searchFilters_example" // String |  (opcjonalny)
let sorts = "sorts_example" // String |  (opcjonalny)
let sso = "sso_example" // String |  (opcjonalny)

ModerationAPI.postApiExport(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]