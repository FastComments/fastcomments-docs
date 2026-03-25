## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## Odgovor

Vraća: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda još su u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (neobavezno)
let mentionGroupIds = ["inner_example"] // [String] |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)
let searchSection = "searchSection_example" // String |  (neobavezno)

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