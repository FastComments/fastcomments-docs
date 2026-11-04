Pripnite anketo k komentarju ali nastavite celotno stanje ankete, ki jo že ima.

Možnosti se ujemajo po id: možnost, poslana z id obstoječe možnosti, obdrži svoje glasove (in prevzame
novo oznako ter položaj), možnost brez id-ja se doda, obstoječe možnosti, ki jih ni na seznamu, pa se odstranijo
skupaj z oddanimi glasovi.

Če na anketi z glasovi ne obdržite nobenega obstoječega id-ja možnosti, se vsi glasovi izbrišejo, zato je treba nastaviti replaceVotes=true.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| replaceVotes | boolean | query | Ne |  |

## Odgovor

Vrne: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer putPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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