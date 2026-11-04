I singoli voti dietro i totali di un sondaggio, dal più vecchio al più recente.

Un sondaggio appartiene a un commento, quindi i voti vengono sempre letti un sondaggio alla volta - commentId è obbligatorio. Questo
garantisce che ogni query utilizzi gli indici già presenti nella collection.

Rispetta la privacy del sondaggio: i voti di un sondaggio anonimo non possono essere letti (poll-anonymous), né qui né per id.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | query | Sì |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Risposta

Restituisce: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (opzionale)
let optionId = "optionId_example" // String |  (opzionale)
let skip = 987 // Int |  (opzionale)

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