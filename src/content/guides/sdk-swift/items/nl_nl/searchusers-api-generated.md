## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nee |  |
| mentionGroupIds | array | query | Nee |  |
| sso | string | query | Nee |  |
| searchSection | string | query | Nee |  |

## Response

Retourneert: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'searchUsers Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld deze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (optioneel)
let mentionGroupIds = ["inner_example"] // [String] |  (optioneel)
let sso = "sso_example" // String |  (optioneel)
let searchSection = "searchSection_example" // String |  (optioneel)

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