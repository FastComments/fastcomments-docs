## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nein |  |
| mentionGroupIds | array | query | Nein |  |
| sso | string | query | Nein |  |
| searchSection | string | query | Nein |  |

## Antwort

Rückgabe: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## Beispiel

[inline-code-attrs-start title = 'searchUsers Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new melden
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (optional)
let mentionGroupIds = ["inner_example"] // [String] |  (optional)
let sso = "sso_example" // String |  (optional)
let searchSection = "searchSection_example" // String |  (optional)

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