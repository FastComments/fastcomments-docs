## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|-----------|-------------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Cuando se establece, devuelve los usuarios para cada id de reacción (avatares y nombres de usuario). Sólo \"public\" es aceptado, lo que omite los perfiles privados. |

## Respuesta

Devuelve: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getV2PageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código están todavía en beta. Para cualquier problema, por favor informe a http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (opcional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Cuando se establece, devuelve los usuarios para cada id de reacción (avatares y nombres de usuario). Sólo \"public\" es aceptado, lo que omite los perfiles privados. (opcional)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
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