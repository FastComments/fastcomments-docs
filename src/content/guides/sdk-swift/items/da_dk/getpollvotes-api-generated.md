The individual votes behind one poll's tallies, oldest first.

En afstemning tilhører en kommentar, så stemmer læses altid én afstemning ad gangen - commentId er påkrævet. Det holder hver forespørgsel på de indekser, som samlingen allerede har.

Overholder afstemningens privatliv: en anonym afstemnings stemmer kan ikke læses (poll-anonymous), her eller via id.

## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Ja |  |
| voterId | string | query | Nej |  |
| optionId | string | query | Nej |  |
| skip | integer | query | Nej |  |

## Svar

Returnerer: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getPollVotes Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optional)
let optionId = "optionId_example" // String |  (optional)
let skip = 987 // Int |  (optional)

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