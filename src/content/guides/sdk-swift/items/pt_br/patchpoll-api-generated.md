Edite uma enquete no local, mantendo suas contagens: altere a pergunta, renomeie uma opção, feche ou reabra a enquete,
ou mude quem pode ver os eleitores. As opções são endereçadas por id - para adicionar, remover ou reordená-las, faça PUT da lista completa.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |

## Resposta

Retorna: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de patchPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
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