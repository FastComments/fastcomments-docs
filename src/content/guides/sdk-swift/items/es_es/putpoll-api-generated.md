Adjunte una encuesta a un comentario, o establezca el estado completo de la encuesta que ya tiene.

Las opciones se emparejan por id: una opción enviada con el id de una opción existente conserva sus votos (y toma la
nueva etiqueta y posición), una opción enviada sin id se agrega, y las opciones existentes que se omiten de la lista
se eliminan junto con los votos emitidos sobre ellas.

Mantener sin ids de opciones existentes en una encuesta que tiene votos elimina todas, por lo que se necesita `replaceVotes=true`.

## Parameters

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| replaceVotes | boolean | query | No |  |

## Respuesta

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código siguen en beta. Para cualquier problema, por favor informe a http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (opcional)

DefaultAPI.putPoll(tenantId: tenantId, commentId: commentId, commentPollPutInput: commentPollPutInput, replaceVotes: replaceVotes) { (response, error) in
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