## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| broadcastId | string | query | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ChangeCommentPinStatusResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unPinComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código todavía están en beta. Para cualquier problema, por favor informe vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let sso = "sso_example" // String |  (opcional)

PublicAPI.unPinComment(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
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