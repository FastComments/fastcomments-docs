## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | רשימת מזהי תגובות מופרדת בפסיקים. |
| sso | string | query | No |  |

## תגובה

מחזיר: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckBlockedCommentsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-checkedCommentsForBlocked'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. עבור כל בעיה, אנא דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | רשימת מזהי תגובות מופרדת בפסיקים.
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.checkedCommentsForBlocked(tenantId: tenantId, commentIds: commentIds, sso: sso) { (response, error) in
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