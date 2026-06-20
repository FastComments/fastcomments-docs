## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIChildCommentsResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentChildren'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getCommentChildren(commentId: commentId, sso: sso) { (response, error) in
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