## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSuggestResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getSearchSuggest'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getSearchSuggest(textSearch: textSearch, sso: sso) { (response, error) in
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