## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |

## תגובה

מחזיר: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת flagComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. עבור כל בעיה, נא לדווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let userId = "userId_example" // String |  (אופציונלי)
let anonUserId = "anonUserId_example" // String |  (אופציונלי)

DefaultAPI.flagComment(tenantId: tenantId, id: id, userId: userId, anonUserId: anonUserId) { (response, error) in
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