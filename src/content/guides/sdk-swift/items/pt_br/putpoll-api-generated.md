---
Anexar uma enquete a um comentário, ou definir o estado completo da enquete que já existe.

As opções são correspondidas por id: uma opção enviada com o id de uma opção existente mantém seus votos (e recebe o novo rótulo e posição), uma opção enviada sem id é adicionada, e opções existentes que não estão na lista são removidas juntamente com os votos lançados nelas.

Não manter nenhum id de opção existente em uma enquete que tem votos exclui todas elas, portanto isso requer replaceVotes=true.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |
| replaceVotes | boolean | query | Não |  |

## Resposta

Retorna: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
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