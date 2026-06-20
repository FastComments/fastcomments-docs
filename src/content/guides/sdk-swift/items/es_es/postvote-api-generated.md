## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| direction | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de postVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor informe a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let direction = "direction_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.postVote(commentId: commentId, direction: direction, sso: sso) { (response, error) in
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