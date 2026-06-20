## Parámetros

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

## Respuesta

Devuelve: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor infórmelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (opcional)
let count = 987 // Double |  (opcional)
let textSearch = "textSearch_example" // String |  (opcional)
let byIPFromComment = "byIPFromComment_example" // String |  (opcional)
let filters = "filters_example" // String |  (opcional)
let searchFilters = "searchFilters_example" // String |  (opcional)
let sorts = "sorts_example" // String |  (opcional)
let demo = true // Bool |  (opcional)
let sso = "sso_example" // String |  (opcional)

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