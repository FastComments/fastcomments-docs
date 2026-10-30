Prikačite anketu uz komentar, ili postavite kompletno stanje ankete koju već ima.

Opcije se podudaraju po id-u: opcija poslata sa id-om postojeće opcije zadržava svoje glasove (i preuzima novi naziv i poziciju), opcija poslata bez id-a se dodaje, a postojeće opcije izostavljene iz liste se uklanjaju zajedno sa glasovima izdatim za njih.

Zadržavanje nijednog postojećeg id-a opcije na anketi koja ima glasove briše sve njih, pa je potrebno postaviti replaceVotes=true.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| replaceVotes | boolean | query | Ne |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Primer

[inline-code-attrs-start title = 'putPoll Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje beta. Za bilo koji problem, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool | (opcionalno)

DefaultAPI.putPoll(tenantId: tenantId, commentId: commentId, commentPollPutInput: commentPollPutInput, replaceVotes: replaceVotes) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]