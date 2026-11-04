Hängt eine Umfrage an einen Kommentar an oder setzt den vollständigen Zustand der Umfrage, die er bereits hat.

Optionen werden anhand der id abgeglichen: Eine Option, die mit der id einer bestehenden Option gesendet wird, behält ihre Stimmen (und übernimmt das neue Label und die neue Position), eine Option ohne id wird hinzugefügt, und bestehende Optionen, die in der Liste fehlen, werden zusammen mit den auf sie abgegebenen Stimmen entfernt.

Wenn keine bestehenden Options-IDs auf einer Umfrage mit Stimmen beibehalten werden, werden alle gelöscht, daher ist dafür replaceVotes=true erforderlich.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| replaceVotes | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'putPoll Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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