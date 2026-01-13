## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| urlId | string | query | Sí |  |

## Respuesta

Devuelve: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetVotes200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'getVotes Ejemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código todavía están en beta. Para cualquier problema, por favor infórmelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 

DefaultAPI.getVotes(tenantId: tenantId, urlId: urlId) { (response, error) in
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