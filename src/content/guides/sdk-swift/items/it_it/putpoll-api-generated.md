Allega un sondaggio a un commento, o imposta lo stato completo del sondaggio che ha già.

Le opzioni vengono abbinate per id: un'opzione inviata con l'id di un'opzione esistente mantiene i suoi voti (e assume la
nuova etichetta e posizione), un'opzione inviata senza id viene aggiunta, e le opzioni esistenti omesse dall'elenco
vengono rimosse insieme ai voti espressi su di esse.

Non mantenere nessun id di opzione esistente su un sondaggio con voti li elimina tutti, quindi per questo è necessario replaceVotes=true.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | path | Sì |  |
| replaceVotes | boolean | query | No |  |

## Risposta

Restituisce: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (opzionale)

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