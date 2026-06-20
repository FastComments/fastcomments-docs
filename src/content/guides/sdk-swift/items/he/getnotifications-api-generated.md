## פרמטרים

| שם | סוג | Location | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| urlId | string | query | לא |  |
| fromCommentId | string | query | לא |  |
| viewed | boolean | query | לא |  |
| type | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (אופציונלי)
let urlId = "urlId_example" // String |  (אופציונלי)
let fromCommentId = "fromCommentId_example" // String |  (אופציונלי)
let viewed = true // Bool |  (אופציונלי)
let type = "type_example" // String |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)

DefaultAPI.getNotifications(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]