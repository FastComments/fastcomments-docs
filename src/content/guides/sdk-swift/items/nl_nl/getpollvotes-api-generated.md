The individual votes behind one poll's tallies, oldest first.

Een poll behoort tot een reactie, dus stemmen worden altijd per poll gelezen - commentId is vereist. Dat houdt elke query op de indexen die de collectie al heeft.

Houdt zich aan de privacy van de poll: de stemmen van een anonieme poll kunnen niet worden gelezen (poll-anonymous), hier of via id.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Ja |  |
| voterId | string | query | Nee |  |
| optionId | string | query | Nee |  |
| skip | integer | query | Nee |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPollVotes Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optioneel)
let optionId = "optionId_example" // String |  (optioneel)
let skip = 987 // Int |  (optioneel)

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