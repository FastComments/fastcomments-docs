Stem op een poll, of verplaats een bestaande stem naar een andere optie. Een stemmer heeft maximaal één stem per
poll, dus het opnieuw aanroepen voor dezelfde stemmer verplaatst hun stem in plaats van er één toe te voegen.

Dit volgt de poll-instellingen van de site: als stemmen alleen is ingesteld voor ingelogde gebruikers, wordt een stem met alleen een
anonUserId geweigerd, en anonieme stemmen worden per IP per poll beperkt in snelheid.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Response

Geeft terug: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'createPollVote Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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