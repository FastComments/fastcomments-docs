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

## Risposta

Restituisce: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (opzionale)
let count = 987 // Double |  (opzionale)
let textSearch = "textSearch_example" // String |  (opzionale)
let byIPFromComment = "byIPFromComment_example" // String |  (opzionale)
let filters = "filters_example" // String |  (opzionale)
let searchFilters = "searchFilters_example" // String |  (opzionale)
let sorts = "sorts_example" // String |  (opzionale)
let demo = true // Bool |  (opzionale)
let sso = "sso_example" // String |  (opzionale)

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