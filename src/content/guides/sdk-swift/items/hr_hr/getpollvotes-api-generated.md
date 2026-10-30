The individual votes behind one poll's tallies, oldest first.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That keeps every query on the indexes the collection already has.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | query | Da |  |
| voterId | string | query | Ne |  |
| optionId | string | query | Ne |  |
| skip | integer | query | Ne |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'Primjer getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (opcionalno)
let optionId = "optionId_example" // String |  (opcionalno)
let skip = 987 // Int |  (opcionalno)

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