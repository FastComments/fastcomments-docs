Uredite anketu na licu mesta, zadržavajući njene rezultate: promenite pitanje, preimenujte opciju, zatvorite ili ponovo otvorite anketu, ili promenite ko može da vidi glasače. Opcije se adresiraju po ID‑u – da biste ih dodali, uklonili ili promenili redosled, pošaljite PUT celog spiska.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |

## Odgovor

Vraća: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Primer

[inline-code-attrs-start title = 'patchPoll Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPatch = CommentPollPatch(question: "question_example", options: [CommentPollPatch_options_inner(label: "label_example", id: "id_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPatch | 

DefaultAPI.patchPoll(tenantId: tenantId, commentId: commentId, commentPollPatch: commentPollPatch) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]