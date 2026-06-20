---
## Paramètres

| Name | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUserInternalProfile'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String |  (optionnel)
let sso = "sso_example" // String |  (optionnel)

ModerationAPI.getUserInternalProfile(commentId: commentId, sso: sso) { (response, error) in
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