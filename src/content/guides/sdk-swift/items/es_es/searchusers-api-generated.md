## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | ruta | Sí |  |
| urlId | string | consulta | Sí |  |
| usernameStartsWith | string | consulta | No |  |
| mentionGroupIds | array | consulta | No |  |
| sso | string | consulta | No |  |
| searchSection | string | consulta | No |  |

## Respuesta

Devuelve: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## Ejemplo

[inline-code-attrs-start title = 'searchUsers Ejemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código están todavía en beta. Para cualquier problema, por favor informe vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (optional)
let mentionGroupIds = ["inner_example"] // [String] |  (optional)
let sso = "sso_example" // String |  (optional)
let searchSection = "searchSection_example" // String |  (optional)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.SearchUsersOptions(usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]