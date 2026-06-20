## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationUserSearchResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getSearchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (facultatif)
let sso = "sso_example" // String |  (facultatif)

ModerationAPI.getSearchUsers(value: value, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]