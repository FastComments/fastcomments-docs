## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Non |  |

## Réponse

Renvoie: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIChildCommentsResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de postCommentsByIds'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentsByIdsParams = CommentsByIdsParams(ids: ["ids_example"]) // CommentsByIdsParams | 
let sso = "sso_example" // String |  (optionnel)

ModerationAPI.postCommentsByIds(commentsByIdsParams: commentsByIdsParams, sso: sso) { (response, error) in
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