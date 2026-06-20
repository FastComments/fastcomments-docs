## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| postIds | array | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UserReactsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserReactsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בגרסת בטא. בכל בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.getUserReactsPublic(tenantId: tenantId, postIds: postIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]