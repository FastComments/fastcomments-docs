Posamezni glasovi za eno anketo, najstarejši najprej.

Anketa pripada komentarju, zato se glasovi vedno berejo po eni anketi naenkrat – commentId je obvezen. To ohranja vsako poizvedbo na indeksih, ki jih zbirka že ima.

Upošteva zasebnost ankete: glasovi anonimne ankete ne morejo biti prebrani (poll-anonymous), tukaj ali po ID-ju.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | query | Da |  |
| voterId | string | query | Ne |  |
| optionId | string | query | Ne |  |
| skip | integer | query | Ne |  |

## Odziv

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Primer

[inline-code-attrs-start title = 'getPollVotes Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za morebitne težave prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (neobvezno)
let optionId = "optionId_example" // String |  (neobvezno)
let skip = 987 // Int |  (neobvezno)

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