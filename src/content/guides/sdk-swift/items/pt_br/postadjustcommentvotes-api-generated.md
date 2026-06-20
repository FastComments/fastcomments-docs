## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AdjustVotesResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postAdjustCommentVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let adjustCommentVotesParams = AdjustCommentVotesParams(adjustVoteAmount: 123) // AdjustCommentVotesParams | 
let sso = "sso_example" // String |  (opcional)

ModerationAPI.postAdjustCommentVotes(commentId: commentId, adjustCommentVotesParams: adjustCommentVotesParams, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]