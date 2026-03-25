## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nej |  |
| mentionGroupIds | array | query | Nej |  |
| sso | string | query | Nej |  |
| searchSection | string | query | Nej |  |

## Svar

Returnerer: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'searchUsers Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (valgfri)
let mentionGroupIds = ["inner_example"] // [String] |  (valgfri)
let sso = "sso_example" // String |  (valgfri)
let searchSection = "searchSection_example" // String |  (valgfri)

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

---