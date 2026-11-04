Priložite anketu komentar ili postavite cijelo stanje ankete koju već ima.

Opcije se usklađuju prema id-u: opcija poslana s id-om postojeće opcije zadržava svoje glasove (i preuzima novu
oznaku i poziciju), opcija poslana bez id-a dodaje se, a postojeće opcije izostavljene s popisa uklanjaju se zajedno s
glasovima danima na njima.

Ako na anketi koja ima glasove ne zadržite nijedan postojeći id opcije, svi se glasovi brišu, pa je za to potrebno replaceVotes=true.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| replaceVotes | boolean | query | Ne |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Primjer

[inline-code-attrs-start title = 'putPoll primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (optional)

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