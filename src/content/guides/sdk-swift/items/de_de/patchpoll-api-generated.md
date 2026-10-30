Edit a poll in place, keeping its tallies: change the question, relabel an option, close or reopen it,
or change who may see the voters. Options are addressed by id - to add, remove or reorder them, PUT the full list.

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |

## Response

Rückgabe: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'patchPoll Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Der folgende Code ist noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
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

---