## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| usernameStartsWith | string | query | Nie |  |
| mentionGroupIds | array | query | Nie |  |
| sso | string | query | Nie |  |
| searchSection | string | query | Nie |  |

## Odpowiedź

Zwraca: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## Przykład

[inline-code-attrs-start title = 'searchUsers Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące próbki kodu są nadal w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (opcjonalny)
let mentionGroupIds = ["inner_example"] // [String] |  (opcjonalny)
let sso = "sso_example" // String |  (opcjonalny)
let searchSection = "searchSection_example" // String |  (opcjonalny)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.SearchUsersOptions(usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]