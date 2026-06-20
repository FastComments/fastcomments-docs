## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentTextResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de postSetCommentText'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo en http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let setCommentTextParams = SetCommentTextParams(comment: "comment_example") // SetCommentTextParams | 
let sso = "sso_example" // String |  (opcional)

ModerationAPI.postSetCommentText(commentId: commentId, setCommentTextParams: setCommentTextParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]