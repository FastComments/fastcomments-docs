Zabilježite glas na anketi ili premjestite postojeći glas na drugu opciju. Glasatelj ima najviše jedan glas po
anketi, tako da ponovni poziv za istog glasatelja premješta njegov glas umjesto da dodaje novi.

Ovo poštuje postavke anketa stranice: ako je glasovanje ograničeno samo na prijavljene korisnike, glas s
samo anonUserId je odbijen, a anonimni glasovi su ograničeni brzinom po IP adresi po anketi.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Primjer

[inline-code-attrs-start title = 'createPollVote Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još uvijek u beti. Za bilo koji problem, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
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