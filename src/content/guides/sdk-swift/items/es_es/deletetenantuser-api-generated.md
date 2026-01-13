## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| deleteComments | string | query | No |  |
| commentDeleteMode | string | query | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteTenantUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Si encuentra algún problema, repórtelo en http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = "deleteComments_example" // String |  (opcional)
let commentDeleteMode = "commentDeleteMode_example" // String |  (opcional)

DefaultAPI.deleteTenantUser(tenantId: tenantId, id: id, deleteComments: deleteComments, commentDeleteMode: commentDeleteMode) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]