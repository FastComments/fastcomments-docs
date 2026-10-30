## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| id | string | query | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteV2PageReact'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código están todavía en beta. Para cualquier problema, por favor informe a http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let id = "id_example" // String | 
let sso = "sso_example" // String |  (opcional)

PublicAPI.deleteV2PageReact(tenantId: tenantId, urlId: urlId, id: id, sso: sso) { (response, error) in
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