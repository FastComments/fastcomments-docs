Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per poll, so calling this again for the same voter moves their vote rather than adding one.

Registre um voto em uma enquete ou mova um voto existente para uma opção diferente. Um eleitor tem no máximo um voto por enquete, portanto chamar este endpoint novamente para o mesmo eleitor move seu voto em vez de adicionar um novo.

This obeys the site's poll settings: if voting is set to logged-in users only, a vote with just an anonUserId is rejected, and anonymous votes are rate limited per IP per poll.

Isso segue as configurações de enquete do site: se a votação estiver configurada apenas para usuários autenticados, um voto com apenas um anonUserId será rejeitado, e votos anônimos são limitados por taxa por IP por enquete.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | query | Yes |  |

## Response

Retorna: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemplo createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
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