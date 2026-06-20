## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | No |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserManualBadgesResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getManualBadgesForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor informe a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgesUserId = "badgesUserId_example" // String |  (opcional)
let commentId = "commentId_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getManualBadgesForUser(badgesUserId: badgesUserId, commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]