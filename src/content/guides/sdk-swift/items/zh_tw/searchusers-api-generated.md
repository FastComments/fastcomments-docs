## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| usernameStartsWith | string | query | 否 |  |
| mentionGroupIds | array | query | 否 |  |
| sso | string | query | 否 |  |
| searchSection | string | query | 否 |  |

## 回應

返回: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## 範例

[inline-code-attrs-start title = 'searchUsers 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍處於測試階段。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (可選)
let mentionGroupIds = ["inner_example"] // [String] |  (可選)
let sso = "sso_example" // String |  (可選)
let searchSection = "searchSection_example" // String |  (可選)

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

---