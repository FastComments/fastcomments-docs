## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| usernameStartsWith | string | query | לא |  |
| mentionGroupIds | array | query | לא |  |
| sso | string | query | לא |  |
| searchSection | string | query | לא |  |

## תגובה

מחזיר: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. עבור כל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (אופציונלי)
let mentionGroupIds = ["inner_example"] // [String] |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)
let searchSection = "searchSection_example" // String |  (אופציונלי)

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