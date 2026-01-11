## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| broadcastId | string | query | Sí |  |
| sessionId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteComment200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de voteComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let voteBodyParams = VoteBodyParams(commenterEmail: "commenterEmail_example", commenterName: "commenterName_example", voteDir: "voteDir_example", url: "url_example") // VoteBodyParams | 
let sessionId = "sessionId_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

PublicAPI.voteComment(tenantId: tenantId, commentId: commentId, urlId: urlId, broadcastId: broadcastId, voteBodyParams: voteBodyParams, sessionId: sessionId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]