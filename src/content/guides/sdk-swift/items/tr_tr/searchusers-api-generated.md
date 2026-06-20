## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| usernameStartsWith | string | query | Hayır |  |
| mentionGroupIds | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| searchSection | string | query | Hayır |  |

## Yanıt

Döndürür: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## Örnek

[inline-code-attrs-start title = 'searchUsers Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta durumundadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (opsiyonel)
let mentionGroupIds = ["inner_example"] // [String] |  (opsiyonel)
let sso = "sso_example" // String |  (opsiyonel)
let searchSection = "searchSection_example" // String |  (opsiyonel)

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