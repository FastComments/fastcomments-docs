The individual votes behind one poll's tallies, oldest first.

I voti individuali dietro i conteggi di un sondaggio, dal più vecchio al più recente.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That
keeps every query on the indexes the collection already has.

Un sondaggio appartiene a un commento, quindi i voti sono sempre letti un sondaggio alla volta - commentId è obbligatorio. Questo mantiene ogni query sugli indici che la collezione possiede già.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

Rispetta la privacy del sondaggio: i voti di un sondaggio anonimo non possono essere letti (poll-anonymous), qui o tramite ID.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | query | Sì |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Response

Restituisce: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'Esempio getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
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

---