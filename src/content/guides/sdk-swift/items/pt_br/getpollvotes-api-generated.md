Os votos individuais por trás dos totais de uma enquete, do mais antigo para o mais recente.

Uma enquete pertence a um comentário, então os votos são sempre lidos uma enquete por vez - commentId é obrigatório. Isso
mantém todas as consultas nos índices que a coleção já possui.

Obedece à privacidade da enquete: os votos de uma enquete anônima não podem ser lidos (poll-anonymous), aqui ou por id.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentId | string | query | Sim |  |
| voterId | string | query | Não |  |
| optionId | string | query | Não |  |
| skip | integer | query | Não |  |

## Resposta

Retorna: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (opcional)
let optionId = "optionId_example" // String |  (opcional)
let skip = 987 // Int |  (opcional)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]