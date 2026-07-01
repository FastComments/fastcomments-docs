## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | consulta | Sí |  |
| commentId | string | consulta | Sí |  |
| direction | string | consulta | Sí |  |
| userId | string | consulta | No |  |
| anonUserId | string | consulta | No |  |

## Respuesta

Devuelve: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Las siguientes muestras de código todavía están en versión beta. Para cualquier problema, por favor infórmelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let direction = "direction_example" // String | 
let userId = "userId_example" // String |  (opcional)
let anonUserId = "anonUserId_example" // String |  (opcional)

DefaultAPI.createVote(tenantId: tenantId, commentId: commentId, direction: direction, options: DefaultAPI.CreateVoteOptions(userId: userId, anonUserId: anonUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]