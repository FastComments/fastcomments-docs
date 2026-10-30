---
Zapišite glas na anketi ali premaknite obstoječ glas na drugo možnost. Glasovalec ima največ en glas na anketo, zato se ob ponovnem klicu za istega glasovalca glas premakne namesto da bi se dodal nov.

To upošteva nastavitve anket na spletnem mestu: če je glasovanje nastavljeno le za prijavljene uporabnike, se glas z le anonUserId zavrne, anonimni glasovi pa so omejeni po IP-ju na anketo.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer createPollVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. V primeru težav jih prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
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

---