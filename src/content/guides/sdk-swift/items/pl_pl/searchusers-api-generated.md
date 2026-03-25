## Parametry

| Name | Type | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| usernameStartsWith | string | query | Nie |  |
| mentionGroupIds | array | query | Nie |  |
| sso | string | query | Nie |  |
| searchSection | string | query | Nie |  |

## Odpowiedź

Zwraca: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w fazie beta. W razie problemu zgłoś to pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (opcjonalne)
let mentionGroupIds = ["inner_example"] // [String] |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)
let searchSection = "searchSection_example" // String |  (opcjonalne)

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