Registra un voto en una encuesta, o mueve uno existente a una opción diferente. Un votante tiene como máximo un voto por
encuesta, por lo que llamar a esto de nuevo para el mismo votante mueve su voto en lugar de añadir uno.

Esto respeta la configuración de encuestas del sitio: si la votación está configurada solo para usuarios conectados, un voto con solo un
anonUserId es rechazado, y los votos anónimos tienen límite de frecuencia por IP por encuesta.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createPollVoteBody = CreatePollVoteBody(commentId: "commentId_example", optionId: "optionId_example", userId: "userId_example", anonUserId: "anonUserId_example", ip: "ip_example") // CreatePollVoteBody | 

DefaultAPI.createPollVote(tenantId: tenantId, createPollVoteBody: createPollVoteBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]