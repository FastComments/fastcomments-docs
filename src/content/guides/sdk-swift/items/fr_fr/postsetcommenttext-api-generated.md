---
## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentTextResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple postSetCommentText'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let setCommentTextParams = SetCommentTextParams(comment: "comment_example") // SetCommentTextParams | 
let sso = "sso_example" // String |  (optionnel)

ModerationAPI.postSetCommentText(commentId: commentId, setCommentTextParams: setCommentTextParams, sso: sso) { (response, error) in
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