## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | Pfad | Ja |  |
| sso | string | Abfrage | Nein |  |

## Antwort

Gibt zurück: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PostRemoveCommentResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'postRemoveComment Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Für Probleme melden Sie sich bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (optional)

ModerationAPI.postRemoveComment(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]