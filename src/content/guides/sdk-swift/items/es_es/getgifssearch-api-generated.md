## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| search | string | query | Sí |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Respuesta

Devuelve: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getGifsSearch'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (opcional)
let rating = "rating_example" // String |  (opcional)
let page = 987 // Double |  (opcional)

PublicAPI.getGifsSearch(tenantId: tenantId, search: search, locale: locale, rating: rating, page: page) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]