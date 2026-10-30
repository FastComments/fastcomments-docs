Edit en afstemning på stedet, mens du bevarer stemmetal: ændr spørgsmålet, omdøb en mulighed, luk eller genåbn den,
eller ændr hvem der må se vælgerne. Mulighederne adresseres via id - for at tilføje, fjerne eller omarrangere dem, PUT den fulde liste.

## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |

## Svar

Returnerer: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'patchPoll Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
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