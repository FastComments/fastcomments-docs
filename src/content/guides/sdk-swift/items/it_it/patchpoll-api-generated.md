Edita un sondaggio in loco, mantenendo i risultati: cambia la domanda, rinomina un'opzione, chiudi o riapri il sondaggio,
oppure modifica chi può vedere i votanti. Le opzioni sono identificate tramite id – per aggiungerle, rimuoverle o riordinarle, esegui un PUT dell'elenco completo.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Example

[inline-code-attrs-start title = 'Esempio patchPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPatch = CommentPollPatch(question: "question_example", options: [CommentPollPatch_options_inner(label: "label_example", id: "id_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPatch | 

DefaultAPI.patchPoll(tenantId: tenantId, commentId: commentId, commentPollPatch: commentPollPatch) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]