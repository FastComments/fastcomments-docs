---
## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| urlId | string | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de putCloseThread'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optionnel)

ModerationAPI.putCloseThread(urlId: urlId, sso: sso) { (response, error) in
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