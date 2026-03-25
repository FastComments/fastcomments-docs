## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## Yanıt

Döndürür: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Örnek

[inline-code-attrs-start title = 'searchUsers Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (isteğe bağlı)
let mentionGroupIds = ["inner_example"] // [String] |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)
let searchSection = "searchSection_example" // String |  (isteğe bağlı)

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