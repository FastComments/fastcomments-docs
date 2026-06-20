## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| dir | integer | query | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentVoteUserNamesSuccessResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentVoteUserNames'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let dir = 987 // Int | 
let sso = "sso_example" // String |  (opcional)

PublicAPI.getCommentVoteUserNames(tenantId: tenantId, commentId: commentId, dir: dir, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]