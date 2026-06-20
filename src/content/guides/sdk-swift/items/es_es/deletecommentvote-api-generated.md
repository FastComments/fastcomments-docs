## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | ruta | Sí |  |
| commentId | string | ruta | Sí |  |
| voteId | string | ruta | Sí |  |
| urlId | string | consulta | Sí |  |
| broadcastId | string | consulta | Sí |  |
| editKey | string | consulta | No |  |
| sso | string | consulta | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteCommentVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código todavía están en beta. Para cualquier problema, repórtelo en http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voteId = "voteId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let editKey = "editKey_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

PublicAPI.deleteCommentVote(tenantId: tenantId, commentId: commentId, voteId: voteId, urlId: urlId, broadcastId: broadcastId, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]