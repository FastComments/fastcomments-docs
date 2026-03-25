## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| usernameStartsWith | string | query | Non |  |
| mentionGroupIds | array | query | Non |  |
| sso | string | query | Non |  |
| searchSection | string | query | Non |  |

## Réponse

Retourne: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (optionnel)
let mentionGroupIds = ["inner_example"] // [String] |  (optionnel)
let sso = "sso_example" // String |  (optionnel)
let searchSection = "searchSection_example" // String |  (optionnel)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]