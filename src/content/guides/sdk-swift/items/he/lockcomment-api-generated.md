## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-lockComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדגימות הבאות של קוד עדיין בבטא. עבור כל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.lockComment(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]