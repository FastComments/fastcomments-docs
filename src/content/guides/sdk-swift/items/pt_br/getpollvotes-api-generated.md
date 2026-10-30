The individual votes behind one poll's tallies, oldest first.

Os votos individuais por trás da contagem de uma enquete, do mais antigo ao mais recente.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

Uma enquete pertence a um comentário, portanto os votos são sempre lidos uma enquete por vez – commentId é obrigatório. Isso mantém cada consulta nos índices que a coleção já possui.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

Obedece a privacidade da enquete: os votos de uma enquete anônima não podem ser lidos (poll-anonymous), aqui ou por ID.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Response

Retorna: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// As amostras de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
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

---