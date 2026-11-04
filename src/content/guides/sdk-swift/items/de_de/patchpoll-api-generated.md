Bearbeiten Sie eine Umfrage direkt, ohne die Stimmen zu verlieren: Ändern Sie die Frage, benennen Sie eine Option um, schließen oder öffnen Sie sie erneut,
oder ändern Sie, wer die Wähler sehen darf. Optionen werden über die id adressiert – um sie hinzuzufügen, zu entfernen oder neu zu ordnen, senden Sie die vollständige Liste per PUT.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |

## Antwort

Gibt zurück: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'patchPoll Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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