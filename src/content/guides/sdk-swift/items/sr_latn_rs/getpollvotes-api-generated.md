Individualni glasovi iza zbirnih rezultata jedne ankete, najstariji prvi.

Anketa pripada komentaru, pa se glasovi uvek čitaju po jednoj anketi – commentId je obavezan. To
drží svaki upit na indeksima koje kolekcija već ima.

Poštuje privatnost ankete: glasovi anonimne ankete ne mogu biti pročitani (poll-anonymous), ni ovde ni po ID-u.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| voterId | string | query | No |  |
| optionId | string | query | No |  |
| skip | integer | query | No |  |

## Response

Vraća: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
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