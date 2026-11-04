Registra un voto su un sondaggio, oppure sposta un voto esistente verso un'opzione diversa. Un votante ha al massimo un voto per
sondaggio, quindi chiamare questa funzione nuovamente per lo stesso votante sposta il suo voto invece di aggiungerne uno.

Rispetta le impostazioni dei sondaggi del sito: se il voto è riservato solo agli utenti registrati, un voto con soltanto un
anonUserId viene rifiutato, e i voti anonimi sono limitati per IP per sondaggio.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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