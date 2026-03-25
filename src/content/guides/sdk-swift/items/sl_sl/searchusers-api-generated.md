## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| usernameStartsWith | string | query | Ne |  |
| mentionGroupIds | array | query | Ne |  |
| sso | string | query | Ne |  |
| searchSection | string | query | Ne |  |

## Odgovor

Vrača: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Primer

[inline-code-attrs-start title = 'searchUsers Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v fazi beta. Za težave poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (neobvezno)
let mentionGroupIds = ["inner_example"] // [String] |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)
let searchSection = "searchSection_example" // String |  (neobvezno)

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