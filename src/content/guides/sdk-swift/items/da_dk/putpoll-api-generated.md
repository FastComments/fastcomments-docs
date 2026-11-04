Vedhæft en afstemning til en kommentar, eller angiv den fulde tilstand for den afstemning, den allerede har.

Indstillinger matches efter id: en indstilling sendt med id'et for en eksisterende indstilling bevarer sine stemmer (og får den nye etiket og position), en indstilling sendt uden id tilføjes, og eksisterende indstillinger, der udelades fra listen, fjernes sammen med de afgivne stemmer på dem.

Hvis man ikke beholder nogen eksisterende indstillings-id'er på en afstemning, der har stemmer, slettes alle af dem, så det kræver replaceVotes=true.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| replaceVotes | boolean | query | Nej |  |

## Svar

Returnerer: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'putPoll Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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