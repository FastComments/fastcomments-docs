## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| urlId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`VotePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VotePollResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio votePoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalalo via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let votePollBody = VotePollBody(optionId: "optionId_example") // VotePollBody | 
let urlId = "urlId_example" // String |  (optional)
let broadcastId = "broadcastId_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

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