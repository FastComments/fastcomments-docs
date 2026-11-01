## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Resposta

Retorna: [`VotePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VotePollResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo votePoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let votePollBody = VotePollBody(optionId: "optionId_example") // VotePollBody | 
let urlId = "urlId_example" // String |  (opcional)
let broadcastId = "broadcastId_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

PublicAPI.votePoll(tenantId: tenantId, commentId: commentId, votePollBody: votePollBody, options: PublicAPI.VotePollOptions(urlId: urlId, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]